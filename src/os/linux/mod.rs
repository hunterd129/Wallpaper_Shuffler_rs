use std::path::Path;
use std::env;

pub mod gnome;
pub mod kde;
pub mod dms;

pub fn set_wallpaper(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let desktop = env::var("XDG_CURRENT_DESKTOP")
        .unwrap_or_else(|_| "Unknown DE/WM".to_string());

    let desktop_lower = desktop.to_lowercase();

    if desktop_lower.contains("kde") || desktop.contains("plasma") {
        kde::set_wallpaper(path)?;
    } else if desktop_lower.contains("gnome") || desktop.contains("ubuntu") {
        gnome::set_wallpaper(path)?;
    } else if desktop_lower.contains("niri") || desktop_lower.contains("hyprland") || desktop_lower.contains("mango") {
        dms::set_wallpaper(path, &desktop)?;
    } else {
        return Err(format!(
                "Unsupported Linux DE/WM detected: '{}'. Only GNOME, KDE, and DMS are currently supported.",
                desktop
        ).into());
    }

    Ok(())
}
