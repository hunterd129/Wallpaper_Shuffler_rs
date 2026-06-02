use std::path::Path;
use std::process::Command;

pub fn set_wallpaper(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let image_path = path.to_str().ok_or("Invalid path")?;
    let file_name = path.file_name().unwrap_or_default().to_string_lossy();

    let uri = format!("file://{}", image_path);

    Command::new("gsettings")
        .args([
            "set",
            "org.gnome.desktop.background",
            "picture-uri",
            &format!("'{}'", uri),
        ])
        .output()?;

    Command::new("gsettings")
        .args([
            "set",
            "org.gnome.desktop.background",
            "picture-uri-dark",
            &format!("'{}'", uri),
        ])
        .output()?;

    println!("SUCCESS: Wallpaper successfully changed for GNOME Desktop via gsettings CLI.");
    println!("Image applied: {}", file_name);
    Ok(())
}
