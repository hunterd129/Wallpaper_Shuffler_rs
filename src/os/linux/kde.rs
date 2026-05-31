use std::path::Path;
use std::process::Command;

pub fn set_wallpaper(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let image_path = path.to_str().ok_or("Invalid path")?;
    let file_name = path.file_name().unwrap_or_default().to_string_lossy();

    let plasma_script = format!(
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
    );

    Command::new("qdbus6")
        .args([
            "org.kde.plasmashell",
            "/PlasmaShell",
            "org.kde.PlasmaShell.evaluateScript",
            &plasma_script,
        ])
        .output()?;

    println!("SUCCESS: Wallpaper successfully changed for KDE Plasma desktop.");
    println!("Image applied: {}", file_name);
    Ok(())
}
