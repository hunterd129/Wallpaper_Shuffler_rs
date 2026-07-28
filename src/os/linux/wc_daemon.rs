use crate::config::AppConfig;
use notify_rust::Notification;
use std::path::Path;
use std::process::Command;
use which::which;

pub fn set_wallpaper(path: &Path, config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    let abs_path = std::fs::canonicalize(path)?;
    let image_path = abs_path.to_str().ok_or("Invalid path string conversion")?;

    let send_error = |msg: &str| {
        let _ = Notification::new()
            .summary("Wall Shuff: Error")
            .body(msg)
            .appname("Wall Shuff")
            .icon("dialog-error")
            .timeout(5000)
            .show();
    };

    let daemon_name = config.wc_daemon.as_str();

    match daemon_name {
        "noctalia" => {
            let output = Command::new("noctalia")
                .args(["msg", "wallpaper-set", image_path])
                .output()?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                let err_msg = format!("Noctalia error: {}", stderr);
                send_error(&err_msg);
                return Err(err_msg.into());
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
                let err_msg = format!("Failed to launch swaybg: {}", e);
                send_error(&err_msg);
                return Err(e.into());
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
                let err_msg = format!("Awww error: {}", stderr);
                send_error(&err_msg);
                return Err(err_msg.into());
            }
        }

        "dms" => {
            if which("dms").is_ok() {
                let output = Command::new("dms")
                    .args(["ipc", "call", "Wallpaper", "set", image_path])
                    .output()?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    let err_msg = format!("DMS IPC error: {}", stderr);
                    send_error(&err_msg);
                    return Err(err_msg.into());
                }
            } else {
                let err_msg = "'dms' executable not found in PATH";
                send_error(err_msg);
                return Err(err_msg.into());
            }
        }

        _ => {
            let err_msg = format!("Unrecognized or unconfigured WC Daemon: '{}'", daemon_name);
            send_error(&err_msg);
            return Err(err_msg.into());
        }
    }

    let genre_name = path
        .parent()
        .and_then(|p| p.file_name())
        .unwrap_or_default()
        .to_string_lossy();
    let file_name = path.file_name().unwrap_or_default().to_string_lossy();

    let body_string = format!("<b>Genre:</b> {} | <b>File:</b> {}", genre_name, file_name);

    let handle = Notification::new()
        .summary("Wallpaper Updated")
        .body(&body_string)
        .appname("Wall Shuff")
        .icon("media-playlist-shuffle")
        .image_path(image_path)
        .action("reshuffle", "Reshuffle")
        .timeout(5000)
        .show()?;

    handle.wait_for_action(|action| {
        if action == "reshuffle" {
            if let Ok(exe) = std::env::current_exe() {
                let _ = Command::new(exe).spawn();
            }
        }
    });

    Ok(())
}
