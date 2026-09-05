use std::path::Path;
use std::process::Command;

pub fn set_wallpaper(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let abs_path = std::fs::canonicalize(path)?;
    let image_path = abs_path.to_str().ok_or("Invalid path")?;

    let set_wall = |key: &str| -> Result<(), Box<dyn std::error::Error>> {
        let status = Command::new("gsettings")
            .args([
                "set",
                "org.gnome.desktop.background",
                key,
                &format!("file://{}", image_path),
            ])
            .status()?;

        if !status.success() {
            return Err(format!("gsettings failed to set {}", key).into());
        }
        Ok(())
    };

    set_wall("picture-uri")?;
    set_wall("picture-uri-dark")?;

    Ok(())
}
