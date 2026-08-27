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
    let desktop_env = std::env::var("XDG_CURRENT_DESKTOP")
        .unwrap_or_else(|_| "unknown".to_string())
        .to_lowercase();

    let backend = match config.de_backend.to_lowercase().as_str() {
        "auto" => {
            if desktop_env.contains("kde") || desktop_env.contains("plasma") {
                "kde"
            } else if desktop_env.contains("gnome") {
                "gnome"
            } else {
                "wc_daemon"
            }
        }
        "gnome" => "gnome",
        "kde" | "plasma" => "kde",
        _ => "wc_daemon",
    };

    match backend {
        "kde" => return kde::set_wallpaper(path, config),
        "gnome" => return gnome::set_wallpaper(path),
        "wc_daemon" if !config.wc_daemon.is_empty() && config.wc_daemon != "N/A" => {
            return wc_daemon::set_wallpaper(path, config);
        }
        _ => {}
    }

    let err_msg = format!(
        "Failed to route wallpaper change. Desktop: '{}', de_backend: '{}', wc_daemon: '{}'.",
        desktop_env, config.de_backend, config.wc_daemon
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
