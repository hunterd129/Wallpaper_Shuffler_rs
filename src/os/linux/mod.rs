use crate::config::AppConfig;
use std::path::Path;
use std::process::Command;

pub mod dms;
pub mod gnome;
pub mod kde;

pub fn get_config_path() -> std::path::PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("/home/hunter/.config"))
        .join("wall_shuff/config.toml")
}

pub fn set_wallpaper(path: &Path, config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    let desktop = std::env::var("XDG_CURRENT_DESKTOP")
        .unwrap_or_else(|_| "unknown".to_string())
        .to_lowercase();

    let image_path = path.to_str().ok_or("Invalid path string conversion")?;

    if desktop.contains("kde") || desktop.contains("plasma") {
        return kde::set_wallpaper(path, config);
    }

    if desktop.contains("gnome") {
        return gnome::set_wallpaper(path);
    }

    match config.wm_backend.as_str() {
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
        }"awww" => {
            Command::new("awww")
                .args(["img", image_path, "--transition-type", "center"])
                .output()?;
            println!("SUCCESS: Swww updated to {}", image_path);
        }
        _ => {
            dms::set_wallpaper(path, &desktop, config)?;
        }
    }

    Ok(())
}
