use std::path::Path;
use std::env;

pub mod gnome;
pub mod kde;
pub mod niri;

pub fn set_wallpaper(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let desktop = env::var("XDG_CURRENT_DESKTOP")
        .unwrap_or_default()
        .to_lowercase();

    if desktop.contains("kde") || desktop.contains("plasma") {
        kde::set_wallpaper(path)?;
    } else if desktop.contains("gnome") || desktop.contains("ubuntu") {
        gnome::set_wallpaper(path)?;
    } else if desktop.contains("niri") {
        niri::set_wallpaper(path)?;
    } else {
        return Err(format!(
                "Unsupported Linux DE/WM detected: '{}'. Only GNOME, KDE, and Niri are currently supported.",
                desktop
        ).into());
    }

    Ok(())
}
