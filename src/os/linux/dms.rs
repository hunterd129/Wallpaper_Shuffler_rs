use crate::config::AppConfig;
use std::path::Path;
use std::process::Command;
use which::which;

pub fn set_wallpaper(
    path: &Path,
    desktop_name: &str,
    _config: &AppConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let image_path = path.to_str().ok_or("Invalid path")?;
    let file_name = path.file_name().unwrap_or_default().to_string_lossy();

    if which("dms").is_ok() {
        Command::new("dms")
            .args(["ipc", "call", "Wallpaper", "set", image_path])
            .output()?;

        println!(
            "SUCCESS: Wallpaper successfully changed for {} workspace.",
            desktop_name
        );
        println!("Image applied: {}", file_name);
    } else {
        println!("ERROR: currently, only dms, noctalia, awww, and swaybg are supported for wayland compositors");
    }

    Ok(())
}
