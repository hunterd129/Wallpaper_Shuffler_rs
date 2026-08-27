#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub fn set_wallpaper(path: &std::path::Path, _config: &crate::config::AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    windows::set_wallpaper(path)
}

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "linux")]
pub use self::linux::set_wallpaper;
