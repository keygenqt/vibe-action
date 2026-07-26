use anyhow::{Result, anyhow};
use fs2::FileExt;
use std::fs::{self, File};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use sysinfo::{Pid, ProcessesToUpdate, System};

use crate::output::output::OutputKind;
use crate::print_text;
use crate::utils::path::cache_dir;

pub struct RunGuard {
    pid_path: PathBuf,
}

impl RunGuard {
    /// Launch the singleton guard: notifies other instances, waits for them to stop,
    /// creates our PID file, starts a stop-file monitor, and returns the guard.
    pub fn start() -> Result<RunGuard> {
        let pid = std::process::id();
        let cache_dir = cache_dir();

        // Ensure directories exist before any operations
        fs::create_dir_all(&cache_dir)?;

        // Prevent concurrent startup by acquiring an exclusive lock file.
        let startup_lock = Self::acquire_startup_lock(&cache_dir)?;

        // Remove stale files (process no longer alive) before doing anything else
        Self::pre_clean(&cache_dir);

        // Notify all running instances by renaming their *.pid → *.pid.stop
        Self::notify_all_instances(&cache_dir, pid);

        // Create our PID file to signal that we are the active instance.
        let pid_path = cache_dir.join(format!("{}.pid", pid));
        File::create(&pid_path)?;

        // Start a background thread that watches for our *.pid.stop and exits when it appears.
        Self::start_stop_monitor(cache_dir.clone(), pid);

        // Wait until all *.pid.stop files disappear (or timeout).
        Self::wait_for_instances(&cache_dir, pid);

        // Forcefully terminate any instances that ignored the shutdown notification and remove their stop files.
        Self::kill_stale_instances(&cache_dir, pid);

        // The guard will delete the pid file on drop (normal or exit)
        drop(startup_lock);
        Ok(RunGuard { pid_path })
    }

    /// Remove *.pid / *.pid.stop files belonging to dead processes.
    fn pre_clean(cache_dir: &PathBuf) {
        if let Ok(entries) = fs::read_dir(cache_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                let is_pid = path.extension().map_or(false, |ext| ext == "pid");
                let is_stop = !is_pid
                    && path.extension().map_or(false, |ext| ext == "stop")
                    && path
                        .file_stem()
                        .map(|s| {
                            std::path::Path::new(s)
                                .extension()
                                .map_or(false, |e| e == "pid")
                        })
                        .unwrap_or(false);

                if !is_pid && !is_stop {
                    continue;
                }

                let pid = if is_pid {
                    path.file_stem()
                        .and_then(|s| s.to_str())
                        .and_then(|s| s.parse::<u32>().ok())
                } else {
                    parse_stop_pid(&path)
                };

                if let Some(pid) = pid {
                    if !Self::pid_exists(pid) {
                        let _ = fs::remove_file(&path);
                    }
                } else {
                    // Malformed name – clean up
                    let _ = fs::remove_file(&path);
                }
            }
        }
    }

    /// Check if a process with given PID exists.
    fn pid_exists(pid: u32) -> bool {
        let mut sys = System::new();
        let target = Pid::from(pid as usize);
        // true ensures dead processes are removed from cache immediately
        sys.refresh_processes(ProcessesToUpdate::Some(&[target]), true);
        sys.process(target).is_some()
    }

    /// Rename all *.pid files to *.pid.stop to notify running instances to shut down.
    fn notify_all_instances(cache_dir: &PathBuf, current_pid: u32) {
        if let Ok(entries) = std::fs::read_dir(&cache_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "pid") {
                    if let Some(pid) = path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .and_then(|s| s.parse::<u32>().ok())
                    {
                        if pid == current_pid {
                            continue;
                        }
                    }
                    let stop_path = path.with_extension("pid.stop");
                    let _ = std::fs::rename(&path, &stop_path);
                }
            }
        }
    }

    /// Wait until all *.pid.stop files disappear or timeout (3 seconds) expires.
    fn wait_for_instances(cache_dir: &PathBuf, current_pid: u32) {
        let timeout = Duration::from_secs(3);
        let deadline = Instant::now() + timeout;
        loop {
            let any_stop = std::fs::read_dir(&cache_dir).ok().map_or(false, |entries| {
                entries.flatten().any(|entry| {
                    let path = entry.path();
                    let matches_pattern = path.extension().map_or(false, |ext| ext == "stop")
                        && path
                            .file_stem()
                            .map_or(false, |stem| stem.to_string_lossy().ends_with(".pid"));

                    if matches_pattern {
                        if let Some(pid) = parse_stop_pid(&path) {
                            return pid != current_pid;
                        }
                    }
                    false
                })
            });
            if !any_stop {
                break;
            }
            if std::time::Instant::now() > deadline {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(200));
        }
    }

    /// Forcefully kill processes that ignored the shutdown notification and remove their stop files.
    fn kill_stale_instances(cache_dir: &PathBuf, current_pid: u32) {
        if let Ok(entries) = std::fs::read_dir(&cache_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "stop")
                    && path
                        .file_stem()
                        .map_or(false, |stem| stem.to_string_lossy().ends_with(".pid"))
                {
                    if let Some(pid) = parse_stop_pid(&path) {
                        if pid == current_pid {
                            continue;
                        }
                        if Self::is_our_process(pid) {
                            let mut sys = System::new();
                            let target_pid = Pid::from(pid as usize);
                            sys.refresh_processes(
                                sysinfo::ProcessesToUpdate::Some(&[target_pid]),
                                true,
                            );
                            if let Some(process) = sys.process(target_pid) {
                                process.kill();
                            }
                        }
                    }
                    let _ = std::fs::remove_file(&path);
                }
            }
        }
    }

    /// Spawn a background thread that watches for our stop file and exits the process when it appears.
    fn start_stop_monitor(cache_dir: PathBuf, pid: u32) {
        std::thread::spawn(move || {
            let stop_path = cache_dir.join(format!("{}.pid.stop", pid));
            let pid_path = cache_dir.join(format!("{}.pid", pid));
            loop {
                std::thread::sleep(std::time::Duration::from_millis(200));
                if stop_path.exists() {
                    print_text!(OutputKind::Warning, "A new instance requested shutdown.");
                    let _ = std::fs::remove_file(&stop_path);
                    let _ = std::fs::remove_file(&pid_path);
                    std::process::exit(0);
                }
            }
        });
    }

    /// Check if a process with the given PID is our own executable by comparing base file names (case-insensitive).
    fn is_our_process(pid: u32) -> bool {
        let our_name = match std::env::current_exe()
            .ok()
            .and_then(|p| p.file_name().map(|n| n.to_string_lossy().into_owned()))
        {
            Some(name) => name,
            None => return false,
        };
        if our_name.is_empty() {
            return false;
        }
        let mut sys = System::new();
        let target = Pid::from(pid as usize);
        sys.refresh_processes(ProcessesToUpdate::Some(&[target]), true);
        sys.process(target)
            .map(|proc| {
                let proc_name = proc.name().to_string_lossy();
                !proc_name.is_empty() && proc_name.eq_ignore_ascii_case(&our_name)
            })
            .unwrap_or(false)
    }

    /// Acquire the startup lock.
    fn acquire_startup_lock(cache_dir: &PathBuf) -> Result<File> {
        let lock_path = cache_dir.join(".run_lock");
        let file =
            File::create(&lock_path).map_err(|e| anyhow!("Failed to open lock file: {}", e))?;
        file.try_lock_exclusive()
            .map_err(|_| anyhow!("Another vibe-action instance is already starting"))?;

        Ok(file)
    }
}

/// Extract PID from a filename like "123.pid.stop".
fn parse_stop_pid(path: &PathBuf) -> Option<u32> {
    path.file_stem()
        .and_then(|s| s.to_str())
        .and_then(|s| s.strip_suffix(".pid"))
        .and_then(|s| s.parse::<u32>().ok())
}

impl Drop for RunGuard {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.pid_path);
    }
}
