#[cfg(target_os = "linux")]
use std::error::Error;

#[cfg(target_os = "linux")]
struct WallShuffTray;

#[cfg(target_os = "linux")]
impl ksni::Tray for WallShuffTray {
    fn id(&self) -> String {
        "com.github.wall_shuff".into()
    }

    fn category(&self) -> ksni::Category {
        ksni::Category::ApplicationStatus
    }

    fn icon_name(&self) -> String {
        "media-playlist-shuffle".into()
    }

    fn title(&self) -> String {
        "Wall Shuff".into()
    }

    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        use ksni::menu::*;
        vec![
            StandardItem {
                label: "Shuffle Now".into(),
                activate: Box::new(|_| {
                    if let Err(e) = wall_shuff::run_shuffle() {
                        eprintln!("Error shuffling: {}", e);
                    }
                }),
                ..Default::default()
            }
            .into(),
            MenuItem::Separator,
            StandardItem {
                label: "Quit".into(),
                activate: Box::new(|_| {
                    std::process::exit(0);
                }),
                ..Default::default()
            }
            .into(),
        ]
    }
}

#[cfg(target_os = "linux")]
fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting wall_shuffd tray daemon...");

    let service = ksni::TrayService::new(WallShuffTray);
    let _handle = service.handle();

    service.spawn();

    let _ = notify_rust::Notification::new()
        .summary("Wall Shuff")
        .body("Daemon started and sitting in system tray.")
        .appname("Wall Shuff")
        .icon("media-playlist-shuffle")
        .timeout(3000)
        .show();

    loop {
        std::thread::sleep(std::time::Duration::from_secs(3600));
    }
}

#[cfg(not(target_os = "linux"))]
fn main() {
    eprintln!("wall_shuffd is only supported on Linux.");
}
