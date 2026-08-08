//! Clipboard modifier — copies value to the system clipboard.

use anyhow::Context;
use anyhow::Result;
use anyhow::bail;
use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;
use image::ImageEncoder;
use image::codecs::png::PngEncoder;
use std::borrow::Cow;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use super::modifier::Modifier;
use super::modifier::ModifierKey;
use crate::configs::app::AppConfig;
use crate::models::context::ContextModel;
use crate::output::format::FormatOutput;
use crate::output::output::OutputKind;
use crate::output::output::OutputType;
use crate::print_text;
use crate::utils;

const ARG_SCREENSHOT: &str = "screenshot";

/// Temp screenshots older than this are cleaned up on the next capture.
const TEMP_TTL: Duration = Duration::from_secs(24 * 60 * 60);

/// Marker error: the user cancelled the interactive selection (Esc).
#[derive(Debug)]
pub struct ScreenshotCancelled;

impl std::fmt::Display for ScreenshotCancelled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Screenshot cancelled by user")
    }
}

impl std::error::Error for ScreenshotCancelled {}

pub struct ClipboardModifier;

/// Provides temporary directory for screenshots.
fn screenshots_dir() -> Result<PathBuf> {
    let dir = std::env::temp_dir().join("vibe_screenshots");
    std::fs::create_dir_all(&dir).context("Failed to create temp screenshot dir")?;
    Ok(dir)
}

/// Best-effort cleanup of screenshots left by previous runs.
fn cleanup_old_screenshots(dir: &Path) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let expired = entry
            .metadata()
            .and_then(|m| m.modified())
            .map(|modified| modified.elapsed().unwrap_or_default() > TEMP_TTL)
            .unwrap_or(false);
        if expired {
            let _ = std::fs::remove_file(entry.path());
        }
    }
}

/// Builds unique timestamped screenshot path with PID.
fn new_screenshot_path() -> Result<PathBuf> {
    let dir = screenshots_dir()?;
    cleanup_old_screenshots(&dir);

    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("System clock is set before 1970")?
        .as_millis();

    // pid protects against concurrent invocations within the same millisecond.
    Ok(dir.join(format!("shot_{millis}_{}.png", std::process::id())))
}

#[cfg(target_os = "linux")]
fn shell_escape(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

#[cfg(target_os = "linux")]
fn on_wayland() -> bool {
    std::env::var_os("WAYLAND_DISPLAY").is_some()
        || std::env::var("XDG_SESSION_TYPE").is_ok_and(|v| v.eq_ignore_ascii_case("wayland"))
}

#[cfg(target_os = "linux")]
fn current_desktop() -> String {
    std::env::var("XDG_CURRENT_DESKTOP")
        .unwrap_or_default()
        .to_ascii_lowercase()
}

/// True if the tool left a non-empty file behind.
fn produced_file(path: &Path) -> bool {
    std::fs::metadata(path)
        .map(|m| m.is_file() && m.len() > 0)
        .unwrap_or(false)
}

/// Candidate screenshot tools, most preferred first.
#[cfg(any(target_os = "macos", target_os = "linux"))]
fn capture_candidates(path: &str) -> Vec<(&'static str, Command)> {
    fn cmd(program: &str, args: &[&str]) -> Command {
        let mut c = Command::new(program);
        c.args(args);
        c
    }

    let mut cmds: Vec<(&'static str, Command)> = Vec::new();

    #[cfg(target_os = "macos")]
    {
        // -i: area selection, -x: no shutter sound.
        cmds.push(("screencapture", cmd("screencapture", &["-i", "-x", path])));
    }

    #[cfg(target_os = "linux")]
    if on_wayland() {
        // 127 from the guard means "dependencies missing, try the next tool";
        // 130 means the user aborted slurp.
        let script = format!(
            "command -v grim >/dev/null 2>&1 && command -v slurp >/dev/null 2>&1 || exit 127; \
             g=$(slurp 2>/dev/null) || exit 130; \
             [ -n \"$g\" ] || exit 130; \
             exec grim -g \"$g\" {}",
            shell_escape(path)
        );
        let grim_slurp = ("grim+slurp", cmd("sh", &["-c", script.as_str()]));
        let gnome = (
            "gnome-screenshot",
            cmd("gnome-screenshot", &["-a", "-f", path]),
        );
        let kde = (
            "spectacle",
            cmd("spectacle", &["-b", "-r", "-n", "-o", path]),
        );

        // Prefer the tool native to the running desktop: an installed foreign
        // tool may launch and then fail at runtime (e.g. grim on GNOME Wayland),
        // which we cannot reliably distinguish from the user cancelling.
        let desktop = current_desktop();
        if desktop.contains("gnome") {
            cmds.extend([gnome, grim_slurp, kde]);
        } else if desktop.contains("kde") || desktop.contains("plasma") {
            cmds.extend([kde, grim_slurp, gnome]);
        } else {
            cmds.extend([grim_slurp, gnome, kde]);
        }
    } else {
        cmds.push(("scrot", cmd("scrot", &["-s", path])));
        cmds.push(("maim", cmd("maim", &["-s", path])));
        cmds.push((
            "gnome-screenshot",
            cmd("gnome-screenshot", &["-a", "-f", path]),
        ));
        cmds.push(("import", cmd("import", &[path])));
    }

    cmds
}

/// Candidate screenshot tools on platforms without interactive capture support.
#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn capture_candidates(_path: &str) -> Vec<(&'static str, Command)> {
    Vec::new()
}

/// Launches a native interactive screenshot tool, returns the saved image path.
fn capture_interactive() -> Result<PathBuf> {
    let path = new_screenshot_path()?;
    let path_str = path
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Temp path contains invalid UTF-8"))?;

    #[cfg(target_os = "linux")]
    if !on_wayland() && std::env::var_os("DISPLAY").is_none() {
        bail!(
            "No display server detected (neither WAYLAND_DISPLAY nor DISPLAY is set). \
             Copy an image to the clipboard instead."
        );
    }

    let candidates = capture_candidates(path_str);
    if candidates.is_empty() {
        bail!(
            "Interactive screenshots are not supported on this platform. Copy an image to the clipboard instead."
        );
    }

    print_text!(
        OutputKind::Info,
        "Clipboard does not contain an image. Please select an area on the screen..."
    );

    let tried: Vec<&str> = candidates.iter().map(|(name, _)| *name).collect();

    for (name, mut cmd) in candidates {
        let status = match cmd.status() {
            Ok(status) => status,
            Err(e) => {
                // Binary not found / not executable — try the next tool.
                print_text!(
                    OutputKind::Debug,
                    "Screenshot tool '{}' is unavailable: {}",
                    name,
                    e
                );
                continue;
            }
        };

        if status.success() && produced_file(&path) {
            return Ok(path);
        }

        if status.code() == Some(127) {
            // Wrapper scripts use 127 to report a missing dependency.
            print_text!(
                OutputKind::Debug,
                "Screenshot tool '{}' is missing dependencies, trying next...",
                name
            );
            continue;
        }

        // The tool launched but produced no file. For interactive tools this
        // almost always means the user aborted the selection (Esc / right-click):
        // screencapture exits 0 without a file, scrot/maim/gnome-screenshot exit
        // non-zero. Falling through to the next candidate would pop up another
        // selection UI right after the user cancelled.
        print_text!(
            OutputKind::Debug,
            "Screenshot tool '{}' exited with {} and produced no file; treating as cancelled",
            name,
            status
        );
        let _ = std::fs::remove_file(&path); // drop a possible empty/partial file
        return Err(ScreenshotCancelled.into());
    }

    bail!(
        "No working screenshot tool found (tried: {}). \
         Install one of them or copy an image to the clipboard.",
        tried.join(", ")
    )
}

/// Loads image from base64 or path.
fn try_load_image_from_value(s: &str) -> Result<Option<image::DynamicImage>> {
    let bytes = if utils::image::is_image(s) {
        BASE64.decode(s).context("Failed to decode base64 image")?
    } else if Path::new(s).exists() {
        std::fs::read(s).with_context(|| format!("Failed to read image file: {s}"))?
    } else {
        return Ok(None);
    };

    let img = image::load_from_memory(&bytes)
        .context("Provided value is expected to be an image, but failed to parse")?;
    Ok(Some(img))
}

/// Copies an image to the clipboard.
fn set_image_to_clipboard(
    clipboard: &mut arboard::Clipboard,
    img: image::DynamicImage,
) -> Result<()> {
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();
    clipboard
        .set_image(arboard::ImageData {
            width: w as usize,
            height: h as usize,
            bytes: Cow::Owned(rgba.into_raw()),
        })
        .context("Failed to set clipboard image")?;
    Ok(())
}

impl Modifier for ClipboardModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Clipboard
    }

    fn apply(&self, value: &ContextModel, arg: &str) -> Result<ContextModel> {
        if arg != ARG_SCREENSHOT {
            if !arg.is_empty() {
                print_text!(
                    OutputKind::Debug,
                    "Unknown clipboard argument '{}'; copying value as text",
                    arg
                );
            }
            if AppConfig::output().output_type() == OutputType::Cli {
                let text =
                    FormatOutput::strip_outer_markdown_blocks(&value.to_string()).to_string();
                let mut clipboard =
                    arboard::Clipboard::new().context("Failed to access clipboard")?;
                clipboard
                    .set_text(&text)
                    .context("Failed to set clipboard")?;
            }
            return Ok(value.clone());
        }

        let s = match value {
            ContextModel::String(s) => s.clone(),
            _ => return Ok(value.clone()),
        };

        let mut clipboard = arboard::Clipboard::new().context("Failed to access clipboard")?;

        // 1. The value itself may be an image (base64 or a file path).
        if let Some(img) = try_load_image_from_value(&s)? {
            set_image_to_clipboard(&mut clipboard, img)?;
            return Ok(value.clone());
        }

        // 2. The clipboard may already hold an image — pass it along as base64 PNG.
        if let Ok(img_data) = clipboard.get_image() {
            let mut png_bytes = Vec::new();
            PngEncoder::new(&mut png_bytes)
                .write_image(
                    &img_data.bytes,
                    img_data.width as u32,
                    img_data.height as u32,
                    image::ExtendedColorType::Rgba8,
                )
                .context("Failed to encode clipboard image as PNG")?;
            return Ok(ContextModel::String(BASE64.encode(&png_bytes)));
        }

        // 3. Otherwise let the user capture a screen area interactively.
        let path = capture_interactive()?;
        let img_bytes = std::fs::read(&path).context("Failed to read captured screenshot")?;
        let img =
            image::load_from_memory(&img_bytes).context("Failed to parse captured screenshot")?;
        set_image_to_clipboard(&mut clipboard, img)?;

        Ok(ContextModel::String(path.to_string_lossy().to_string()))
    }
}
