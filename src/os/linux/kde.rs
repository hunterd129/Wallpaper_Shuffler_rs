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

fn run_cmd_with_fallback(
    primary: &str,
    fallback: &str,
    args: &[&str],
) -> Result<std::process::Output, std::io::Error> {
    match Command::new(primary).args(args).output() {
        Ok(output) => Ok(output),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            Command::new(fallback).args(args).output()
        }
        Err(e) => Err(e),
    }
}

pub fn set_wallpaper(path: &Path, config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    let abs_path = std::fs::canonicalize(path)?;
    let image_path = abs_path.to_str().ok_or("Invalid path")?;

    let script = build_plasma_script(image_path);
    let qdbus_args = [
        "org.kde.plasmashell",
        "/PlasmaShell",
        "org.kde.PlasmaShell.evaluateScript",
        &script,
    ];

    let output = run_cmd_with_fallback("qdbus6", "qdbus", &qdbus_args)?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("qdbus evaluation failed: {}", stderr).into());
    }

    if config.kde.lockscreen_support {
        let lock_args = [
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
        ];

        let lock_res = run_cmd_with_fallback("kwriteconfig6", "kwriteconfig5", &lock_args);
        if let Err(e) = lock_res {
            eprintln!("Warning: Failed to update KDE lockscreen: {}", e);
        }
    }

    Ok(())
}
