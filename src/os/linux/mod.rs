use crate::config::AppConfig;
use notify_rust::Notification;
use std::path::Path;

pub mod gnome;
pub mod kde;
pub mod wc_daemon;

fn send_success_notification(path: &Path) {
    let abs_path = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    let image_path = abs_path.to_str().unwrap_or_default();

    let genre_name = path
        .parent()
        .and_then(|p| p.file_name())
        .unwrap_or_default()
        .to_string_lossy();
    let file_name = path.file_name().unwrap_or_default().to_string_lossy();

    let body_string = format!("<b>Genre:</b> {} | <b>File:</b> {}", genre_name, file_name);

    let _ = Notification::new()
        .summary("Wallpaper Updated")
        .body(&body_string)
        .appname("Wall Shuff")
        .icon("media-playlist-shuffle")
        .image_path(image_path)
        .timeout(5000)
        .show();
}

fn send_error_notification(err_msg: &str) {
    let _ = Notification::new()
        .summary("Wall Shuff: Error")
        .body(err_msg)
        .appname("Wall Shuff")
        .icon("dialog-error")
        .timeout(5000)
        .show();
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

    let result = match backend {
        "kde" => kde::set_wallpaper(path, config),
        "gnome" => gnome::set_wallpaper(path),
        "wc_daemon" if !config.wc_daemon.is_empty() && config.wc_daemon != "N/A" => {
            wc_daemon::set_wallpaper(path, config)
        }
        _ => {
            let msg = format!(
                "Failed to route wallpaper change. Desktop: '{}', de_backend: '{}', wc_daemon: '{}'.",
                desktop_env, config.de_backend, config.wc_daemon
            );
            Err(msg.into())
        }
    };

    match result {
        Ok(_) => {
            send_success_notification(path);
            Ok(())
        }
        Err(e) => {
            send_error_notification(&e.to_string());
            Err(e)
        }
    }
}
