use crate::config::AppConfig;
use std::path::Path;
use std::process::Command;

fn build_plasma_script(image_path: &str) -> String {
    format!(
        r#"
        var Desktops = desktops();
        for (var i = 0; i < Desktops.length; i++) {{
            var d = Desktops[i];
            d.wallpaperPlugin = "org.kde.image";
            d.currentConfigGroup = new Array("Wallpaper", "org.kde.image", "General");
            d.writeConfig("Image", "file://{}");
        }}
        "#,
        image_path
    )
}

pub fn set_wallpaper(path: &Path, config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    let image_path = path.to_str().ok_or("Invalid path")?;
    let file_name = path.file_name().unwrap_or_default().to_string_lossy();

    Command::new("qdbus6")
        .args([
            "org.kde.plasmashell",
            "/PlasmaShell",
            "org.kde.PlasmaShell.evaluateScript",
            &build_plasma_script(image_path),
        ])
        .output()?;

    println!("SUCCESS: Wallpaper successfully changed for KDE Plasma desktop");

    if config.kde.lockscreen_support {
        Command::new("kwriteconfig6")
            .args([
                "--file",
                "kscreenlockerrc",
                "--group",
                "Greeter",
                "--group",
                "Wallpaper",
                "--group",
                "org.kde.image",
                "--group",
                "General",
                "--key",
                "Image",
                &format!("file://{}", image_path),
            ])
            .output()?;

        println!("SUCCESS: KDE Lockscreen wallpaper successfully synchronized");
    }

    println!("Image applied: {}", file_name);
    Ok(())
}
