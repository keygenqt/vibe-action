use anyhow::{Context, Result};
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use std::fs::{self, File, OpenOptions};
use std::time::Duration;

use crate::utils::path::config_dir;

/// Single active-run record for singleton CLI execution.
/// Holds the PID of the currently running top-level process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveRun {
    pub pid: u32,
}

/// Guard holding the exclusive lock for the whole process lifetime.
/// Released on drop (normal exit, unwind). The OS also releases the lock
/// if the process is SIGKILLed, so the lock — unlike the JSON file — never goes stale.
pub struct ActiveRunGuard {
    _lock_file: File, // held to keep the flock alive
}

impl ActiveRun {
    /// Create a new record for the current process.
    fn new() -> Self {
        Self {
            pid: std::process::id(),
        }
    }

    /// Acquire exclusive execution.
    ///
    /// Fails immediately if another instance already holds the lock — concurrent
    /// top-level launches are rejected, not queued. Otherwise, under the lock,
    /// terminates the previous run recorded in the PID file and writes its own
    /// record. The returned guard must be kept alive for the duration of the run;
    /// dropping it releases the lock and removes the PID file.
    ///
    /// Subprocesses spawned by an already-locked parent (e.g. bench cases) must
    /// skip this via `VIBE_SKIP_LOCK` — the parent holds the lock on their behalf.
    pub fn acquire() -> Result<ActiveRunGuard> {
        // 1. Take the exclusive lock. Fails immediately if another instance
        //    already holds it — concurrent top-level launches are rejected,
        //    not queued.
        let lock_file = Self::lock_file()?;
        lock_file
            .try_lock_exclusive()
            .context("Another vibe-action instance is starting up")?;

        // 2. Under the lock: terminate the previous run, if any.
        match Self::read() {
            Ok(Some(prev)) => {
                if let Err(e) = prev.terminate() {
                    eprintln!("Warning: could not terminate previous process: {}", e);
                }
            }
            Ok(None) => {}
            Err(e) => {
                // Corrupt/unreadable record file — log and self-heal by overwriting.
                eprintln!("Warning: could not read active-run file: {}", e);
            }
        }

        // 3. Record ourselves as the active run.
        let this = Self::new();
        this.save()?;

        Ok(ActiveRunGuard {
            _lock_file: lock_file,
        })
    }

    /// Read the active-run record from disk, if present and valid.
    fn read() -> Result<Option<Self>> {
        let path = Self::path();
        if !path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&path)?;
        if content.trim().is_empty() {
            return Ok(None);
        }

        let record: ActiveRun =
            serde_json::from_str(&content).with_context(|| "Failed to parse active-run file")?;

        // Verify the process is alive and belongs to us.
        if Self::is_our_process(record.pid) {
            Ok(Some(record))
        } else {
            // Stale entry – remove the file.
            fs::remove_file(&path).ok();
            Ok(None)
        }
    }

    /// Atomically write the current record to disk.
    fn save(&self) -> Result<()> {
        let path = Self::path();
        let tmp = path.with_extension("tmp");
        let json = serde_json::to_string(self)?;
        fs::write(&tmp, json)?;
        fs::rename(&tmp, &path)?;
        Ok(())
    }

    /// Remove the active-run file.
    fn remove() -> Result<()> {
        let path = Self::path();
        if path.exists() {
            fs::remove_file(&path)?;
        }
        Ok(())
    }

    /// Try to terminate a previous instance gracefully, then forcefully.
    fn terminate(&self) -> Result<()> {
        if !Self::is_our_process(self.pid) {
            return Ok(());
        }

        // Graceful signal. On Unix = SIGTERM; on Windows there is no graceful
        // cross-process signal, so this returns `false` and we go straight to kill.
        let graceful_sent = Self::send_terminate(self.pid);

        if graceful_sent {
            // Wait a grace period for the process to exit and release the cluster.
            let grace = Duration::from_secs(5);
            let start = std::time::Instant::now();
            while start.elapsed() < grace {
                if !Self::is_our_process(self.pid) {
                    return Ok(());
                }
                std::thread::sleep(Duration::from_millis(100));
            }
        }

        // Force kill if still alive (or if graceful is unsupported on this platform).
        Self::force_kill(self.pid)
    }

    /// Check that the PID belongs to a live process of our own executable
    /// (name match). Returns false for foreign or dead processes — this guards
    /// against acting on a stale PID reused by an unrelated process.
    fn is_our_process(pid: u32) -> bool {
        if !Self::pid_exists(pid) {
            return false;
        }

        // Name must match our executable.
        let name = Self::process_name(pid);
        let our_name = std::env::current_exe()
            .ok()
            .and_then(|p| p.file_name().map(|n| n.to_string_lossy().into_owned()))
            .unwrap_or_else(|| "vibe-action".to_string());

        name.map_or(false, |n| n == our_name)
    }

    /// Check whether a process with the given PID exists in the system.
    fn pid_exists(pid: u32) -> bool {
        #[cfg(unix)]
        {
            unsafe { libc::kill(pid as i32, 0) == 0 }
        }
        #[cfg(windows)]
        {
            use windows_sys::Win32::System::Threading::{
                OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION,
            };
            let handle = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid) };
            if handle == 0 {
                false
            } else {
                unsafe { windows_sys::Win32::Foundation::CloseHandle(handle) };
                true
            }
        }
    }

    /// Retrieve the executable name of a process by PID (platform-specific).
    fn process_name(pid: u32) -> Option<String> {
        #[cfg(target_os = "linux")]
        {
            std::fs::read_to_string(format!("/proc/{}/comm", pid))
                .ok()
                .map(|s| s.trim().to_string())
        }
        #[cfg(target_os = "macos")]
        {
            std::process::Command::new("ps")
                .args(["-p", &pid.to_string(), "-o", "comm="])
                .output()
                .ok()
                .and_then(|out| String::from_utf8(out.stdout).ok())
                .map(|s| s.trim().to_string())
        }
        #[cfg(windows)]
        {
            use windows_sys::Win32::System::ProcessStatus::GetModuleBaseNameA;
            use windows_sys::Win32::System::Threading::{
                OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ,
            };
            let handle =
                unsafe { OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, pid) };
            if handle == 0 {
                return None;
            }
            let mut buf = [0u8; 260];
            let len = unsafe {
                GetModuleBaseNameA(
                    handle,
                    std::ptr::null_mut(),
                    buf.as_mut_ptr(),
                    buf.len() as u32,
                )
            };
            unsafe { windows_sys::Win32::Foundation::CloseHandle(handle) };
            if len > 0 {
                Some(String::from_utf8_lossy(&buf[..len as usize]).into_owned())
            } else {
                None
            }
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos", windows)))]
        {
            None
        }
    }

    /// Try to send a graceful termination signal.
    /// Returns `true` if a graceful signal was delivered (Unix SIGTERM),
    /// `false` when the platform has no graceful cross-process signal (Windows).
    fn send_terminate(pid: u32) -> bool {
        #[cfg(unix)]
        {
            unsafe { libc::kill(pid as i32, libc::SIGTERM) == 0 }
        }
        #[cfg(windows)]
        {
            let _ = pid;
            // Windows has no SIGTERM equivalent for arbitrary processes.
            // CTRL_BREAK/CTRL_C only work within the same console group, which is
            // unreliable when spawned from a GUI plugin — so we report "not supported"
            // and the caller proceeds straight to TerminateProcess.
            false
        }
    }

    /// Forcefully terminate a process (SIGKILL on Unix, TerminateProcess on Windows).
    fn force_kill(pid: u32) -> Result<()> {
        #[cfg(unix)]
        {
            if unsafe { libc::kill(pid as i32, libc::SIGKILL) } != 0 {
                return Err(anyhow::anyhow!(
                    "SIGKILL failed: {}",
                    std::io::Error::last_os_error()
                ));
            }
            Ok(())
        }
        #[cfg(windows)]
        {
            use windows_sys::Win32::System::Threading::{
                OpenProcess, PROCESS_TERMINATE, TerminateProcess,
            };
            let handle = unsafe { OpenProcess(PROCESS_TERMINATE, 0, pid) };
            if handle == 0 {
                return Err(anyhow::anyhow!("Failed to open process"));
            }
            let result = unsafe { TerminateProcess(handle, 1) };
            unsafe { windows_sys::Win32::Foundation::CloseHandle(handle) };
            if result == 0 {
                Err(anyhow::anyhow!("TerminateProcess failed"))
            } else {
                Ok(())
            }
        }
    }

    /// Path to the JSON record file.
    fn path() -> std::path::PathBuf {
        config_dir().join("active.json")
    }

    /// Path to the lock file (separate from the JSON record).
    fn lock_path() -> std::path::PathBuf {
        config_dir().join("active.lock")
    }

    /// Open (create if needed) the lock file.
    fn lock_file() -> Result<File> {
        let path = Self::lock_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)
            .with_context(|| format!("Failed to open lock file: {}", path.display()))
    }
}

/// On drop: remove the PID file, then release the lock (via dropping `_lock_file`).
/// Note: Drop does NOT run on SIGKILL — but the OS releases the file lock anyway,
/// and the stale JSON record is cleaned up by validation in `read()` on next start.
impl Drop for ActiveRunGuard {
    fn drop(&mut self) {
        let _ = ActiveRun::remove();
        // `_lock_file` is dropped after this, releasing the flock.
    }
}
