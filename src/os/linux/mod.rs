use crate::config::AppConfig;
use notify_rust::Notification;
use std::path::Path;

pub mod gnome;
pub mod kde;
pub mod wc_daemon;

pub fn get_config_path() -> std::path::PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("~/.config"))
        .join("wall_shuff/config.toml")
}

pub fn set_wallpaper(path: &Path, config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    let desktop = std::env::var("XDG_CURRENT_DESKTOP")
        .unwrap_or_else(|_| "unknown".to_string())
        .to_lowercase();

    if desktop.contains("kde") || desktop.contains("plasma") {
        return kde::set_wallpaper(path, config);
    }

    if desktop.contains("gnome") {
        return gnome::set_wallpaper(path);
    }

    if !config.wc_daemon.is_empty() {
        return wc_daemon::set_wallpaper(path, config);
    }

    let err_msg = format!(
        "Unsupported or unrecognized desktop environment: '{}'. Please configure 'wc_daemon' in config.toml.",
        desktop
    );

    let _ = Notification::new()
        .summary("Wall Shuff: Error")
        .body(&err_msg)
        .appname("Wall Shuff")
        .icon("dialog-error")
        .timeout(5000)
        .show();

    Err(err_msg.into())
}
