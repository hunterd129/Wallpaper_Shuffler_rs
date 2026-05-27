use std::path::Path;
use gio::prelude::SettingsExt;
use gio::Settings;

pub fn set_wallpaper(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let image_path = path.to_str().ok_or("Invalid path")?;
    let image_uri = format!("file://{}", image_path);

    let settings = Settings::new("org.gnome.desktop.background");
    settings.set_string("picture-uri", &image_uri)?;
    settings.set_string("picture-uri-dark", &image_uri)?;

    let file_name = path.file_name().unwrap_or_default().to_string_lossy();
    println!("SUCCESS: Wallpaper successfully shuffled");
    println!("File applied: {}", file_name);

    Ok(())
}

