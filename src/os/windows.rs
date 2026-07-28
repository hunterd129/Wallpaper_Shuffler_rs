use crate::config::AppConfig;
use notify_rust::Notification;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use windows::Win32::UI::WindowsAndMessaging::{
    SystemParametersInfoW, SPIF_UPDATEINIFILE, SPI_SETDESKWALLPAPER,
};

pub fn set_wallpaper(path: &Path, _config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    let path_wide: Vec<u16> = OsStr::new(path.as_os_str())
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    let result = unsafe {
        SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            Some(path_wide.as_ptr() as *mut _),
            SPIF_UPDATEINIFILE,
        )
    };

    if let Err(e) = result {
        let _ = Notification::new()
            .summary("Wall Shuff: Error")
            .body(&format!("Failed to set wallpaper: {}", e))
            .appname("Wall Shuff")
            .icon("dialog-error")
            .timeout(5000)
            .show();

        return Err(Box::new(e));
    }

    let genre_name = path
        .parent()
        .and_then(|p| p.file_name())
        .unwrap_or_default()
        .to_string_lossy();
    let file_name = path.file_name().unwrap_or_default().to_string_lossy();

    let body_string = format!("Genre: {}\nFile: {}", genre_name, file_name);

    let mut notification = Notification::new();
    notification
        .summary("Wallpaper Updated")
        .body(&body_string)
        .appname("Wall Shuff")
        .icon("media-playlist-shuffle")
        .timeout(5000);

    if let Ok(abs_path) = std::fs::canonicalize(path) {
        if let Some(path_str) = abs_path.to_str() {
            notification.image_path(path_str);
        }
    }

    let _ = notification.show();

    Ok(())
}
