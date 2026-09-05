use crate::config::AppConfig;
use std::path::Path;
use std::process::Command;
use which::which;

pub fn set_wallpaper(path: &Path, config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    let abs_path = std::fs::canonicalize(path)?;
    let image_path = abs_path.to_str().ok_or("Invalid path string conversion")?;

    let daemon_name = config.wc_daemon.as_str();

    match daemon_name {
        "noctalia" => {
            let output = Command::new("noctalia")
                .args(["msg", "wallpaper-set", image_path])
                .output()?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(format!("Noctalia error: {}", stderr).into());
            }
        }

        "swaybg" => {
            let old_pids: Vec<String> =
                String::from_utf8_lossy(&Command::new("pidof").arg("swaybg").output()?.stdout)
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect();

            let spawn_result = Command::new("swaybg")
                .args(["-i", image_path, "-m", "fill"])
                .stdin(std::process::Stdio::null())
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn();

            if let Err(e) = spawn_result {
                return Err(format!("Failed to launch swaybg: {}", e).into());
            }

            std::thread::sleep(std::time::Duration::from_millis(150));
            for pid in old_pids {
                let _ = Command::new("kill").arg(&pid).output();
            }
        }

        "awww" => {
            let output = Command::new("awww")
                .args(["img", image_path, "--transition-type", "center"])
                .output()?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(format!("Awww error: {}", stderr).into());
            }
        }

        "dms" => {
            if which("dms").is_ok() {
                let output = Command::new("dms")
                    .args(["ipc", "wallpaper", "set", image_path])
                    .output()?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(format!("DMS IPC error: {}", stderr).into());
                }
            } else {
                return Err("'dms' executable not found in PATH".into());
            }
        }

        _ => {
            return Err(
                format!("Unrecognized or unconfigured WC Daemon: '{}'", daemon_name).into(),
            );
        }
    }

    Ok(())
}
