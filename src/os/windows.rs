use crate::config::AppConfig;
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
    unsafe {
        SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            Some(path_wide.as_ptr() as *mut _),
            SPIF_UPDATEINIFILE,
        )?;
    }
    Ok(())
}
