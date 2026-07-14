use crate::config::AppConfig;
use std::path::Path;
use std::process::Command;
use which::which;

pub fn set_wallpaper(path: &Path, config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    let image_path = path.to_str().ok_or("Invalid path string conversion")?;
    let file_name = path.file_name().unwrap_or_default().to_string_lossy();

    match config.wc_daemon.as_str() {
        "noctalia" => {
            Command::new("noctalia")
                .args(["msg", "wallpaper-set", image_path])
                .output()?;
            println!("SUCCESS: Noctalia updated to {}", image_path);
        }

        "swaybg" => {
            let old_pids: Vec<String> =
                String::from_utf8_lossy(&Command::new("pidof").arg("swaybg").output()?.stdout)
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect();

            Command::new("swaybg")
                .args(["-i", image_path, "-m", "fill"])
                .stdin(std::process::Stdio::null())
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn()?;

            std::thread::sleep(std::time::Duration::from_millis(150));
            for pid in old_pids {
                let _ = Command::new("kill").arg(&pid).output();
            }
            println!("SUCCESS: Swaybg refreshed to {}", image_path);
        }

        "awww" => {
            Command::new("awww")
                .args(["img", image_path, "--transition-type", "center"])
                .output()?;
            println!("SUCCESS: Awww updated to {}", image_path);
        }

        "dms" => {
            if which("dms").is_ok() {
                Command::new("dms")
                    .args(["ipc", "call", "Wallpaper", "set", image_path])
                    .output()?;
                println!("SUCCESS: Wallpaper successfully changed via DMS.");
                println!("Image applied: {}", file_name);
            } else {
                println!("ERROR: 'dms' executable not found in PATH.");
            }
        }

        _ => {
            println!("ERROR: Unrecognized or unconfigured WC Daemon: '{}'", config.wc_daemon);
        }
    }

    Ok(())
}
