use std::path::Path;
use std::process::Command;

pub fn set_wallpaper(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let image_path = path.to_str().ok_or("Invalid path")?;
    let file_name = path.file_name().unwrap_or_default().to_string_lossy();

    let set_wall = |key: &str| -> std::io::Result<()> {
        Command::new("gsettings")
            .args([
                "set",
                "org.gnome.desktop.background",
                key,
                &format!("file://{}", image_path),
            ])
            .status()?;
        Ok(())
    };

    set_wall("picture-uri")?;
    set_wall("picture-uri-dark")?;

    println!("SUCCESS: Wallpaper successfully changed for GNOME Desktop.");
    println!("Image applied: {}", file_name);
    Ok(())
}
