use std::path::Path;
use std::process::Command;

pub fn set_wallpaper(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let image_path = path.to_str().ok_or("Invalid path")?;
    let file_name = path.file_name().unwrap_or_default().to_string_lossy();

    let has_dms = Command::new("which")
        .arg("dms")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if has_dms {
        Command::new("dms")
            .args(["ipc" , "wallpaper", "set", image_path])
            .output()?;

        println!("SUCCESS: Wallpaper successfully changed for Niri (DMS) Wayland Compositor");
        println!("Image applied: {}", file_name);
    } else {
        println!("FAILURE: Currently, only dank material shell is supported.");
    }
    
    Ok(())
}
