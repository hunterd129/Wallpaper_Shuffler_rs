use crate::config::AppConfig;
use notify_rust::Notification;
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
    let abs_path = std::fs::canonicalize(path)?;
    let image_path = abs_path.to_str().ok_or("Invalid path")?;

    let output = Command::new("qdbus6")
        .args([
            "org.kde.plasmashell",
            "/PlasmaShell",
            "org.kde.PlasmaShell.evaluateScript",
            &build_plasma_script(image_path),
        ])
        .output();

    match output {
        Ok(out) if !out.status.success() => {
            let stderr = String::from_utf8_lossy(&out.stderr);
            let err_msg = format!("qdbus6 evaluation failed: {}", stderr);
            let _ = Notification::new()
                .summary("Wall Shuff: Error")
                .body(&err_msg)
                .appname("Wall Shuff")
                .icon("dialog-error")
                .timeout(5000)
                .show();
            return Err(err_msg.into());
        }
        Err(e) => {
            let err_msg = format!("Failed to run qdbus6: {}", e);
            let _ = Notification::new()
                .summary("Wall Shuff: Error")
                .body(&err_msg)
                .appname("Wall Shuff")
                .icon("dialog-error")
                .timeout(5000)
                .show();
            return Err(Box::new(e));
        }
        _ => {}
    }

    if config.kde.lockscreen_support {
        let lock_status = Command::new("kwriteconfig6")
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
            .status();

        if let Err(e) = lock_status {
            eprintln!("Warning: Failed to update KDE lockscreen: {}", e);
        }
    }

    let genre_name = path
        .parent()
        .and_then(|p| p.file_name())
        .unwrap_or_default()
        .to_string_lossy();
    let file_name = path.file_name().unwrap_or_default().to_string_lossy();

    let body_string = format!("<b>Genre:</b> {} | <b>File:</b> {}", genre_name, file_name);

    let handle = Notification::new()
        .summary("Wallpaper Updated")
        .body(&body_string)
        .appname("Wall Shuff")
        .icon("media-playlist-shuffle")
        .image_path(image_path)
        .action("reshuffle", "Reshuffle")
        .timeout(5000)
        .show()?;

    handle.wait_for_action(|action| {
        if action == "reshuffle" {
            if let Ok(exe) = std::env::current_exe() {
                let _ = Command::new(exe).spawn();
            }
        }
    });

    Ok(())
}
