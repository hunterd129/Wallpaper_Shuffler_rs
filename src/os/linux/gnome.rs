use notify_rust::Notification;
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

    if let Err(e) = set_wall("picture-uri").and_then(|_| set_wall("picture-uri-dark")) {
        let _ = Notification::new()
            .summary("Wall Shuff: Error")
            .body(&format!("GNOME Backend Error: {}", e))
            .appname("Wall Shuff")
            .icon("dialog-error")
            .timeout(5000)
            .show();

        return Err(e);
    }

    let genre_name = path
        .parent()
        .and_then(|p| p.file_name())
        .unwrap_or_default()
        .to_string_lossy();
    let file_name = path.file_name().unwrap_or_default().to_string_lossy();

    let body_string = format!("<b>Genre:</b> {} | <b>File:</b> {}", genre_name, file_name);

    Notification::new()
        .summary("Wallpaper Updated")
        .body(&body_string)
        .appname("Wall Shuff")
        .icon("media-playlist-shuffle")
        .image_path(image_path)
        .timeout(5000)
        .show()?;

    Ok(())
}
