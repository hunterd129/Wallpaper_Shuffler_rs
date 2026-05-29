use std::path::Path;
use std::process::Command;
use std::env;
use gio::prelude::SettingsExt;
use gio::Settings;

pub fn set_wallpaper(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let image_path = path.to_str().ok_or("Invalid path")?;
    let file_name = path.file_name().unwrap_or_default().to_string_lossy();

    let desktop = env::var("XDG_CURRENT_DESKTOP")
        .unwrap_or_default()
        .to_lowercase();

    //TODO: test on kde DE before pushing change.
    if desktop.contains("kde") || desktop.contains("plasma") {
        let plasma_script = format!(
            r#"
                var Desktops = desktops();
            for (var i = 0; i < Desktops.length; i++) {{
                var d = Desktops[i];
                d.currentConfigGroup = new Array("Background", "org.kde.image", "General");
                d.writeConfig("Image", "file://{}");
            }}
            "#,
            image_path
        );

        Command::new("qdbus")
            .args([
                "org.kde.plasmashell",
                "/PlasmaShell",
                "org.kde.PlasmaShell.evaluateScript",
                &plasma_script,
            ])
            .output()?;

        println!("SUCCESS: Wallpaper successfully changed for KDE Plasma desktop.");
        println!("File applied: {}", file_name);
    
    } else if desktop.contains("gnome") || desktop.contains("ubuntu") {
        let image_uri = format!("file://{}", image_path);
        let settings = Settings::new("org.gnome.desktop.background");
        settings.set_string("picture-uri", &image_uri)?;
        settings.set_string("picture-uri-dark", &image_uri)?;

        println!("SUCCESS: Wallpaper successfully change for GNOME desktop.");
        println!("File applied: {}", file_name);
    } else {
        return Err(format!(
            "Unsupported Linux desktop environment/window manager detected: '{}'. Only GNOME and KDE are currently supported.",
            desktop
        ).into());
    }

    Ok(())
}
