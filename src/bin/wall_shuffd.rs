#[cfg(target_os = "linux")]
use std::error::Error;
#[cfg(target_os = "linux")]
use std::time::Duration;

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
fn duration_until_midnight() -> Duration {
    use chrono::Local;

    let now = Local::now();
    let tomorrow = (now.date_naive() + chrono::Days::new(1))
        .and_hms_opt(0, 0, 0)
        .unwrap();

    let now_naive = now.naive_local();
    let duration = tomorrow - now_naive;

    Duration::from_secs(duration.num_seconds().max(1) as u64)
}

#[cfg(target_os = "linux")]
fn main() -> Result<(), Box<dyn Error>> {
    let service = ksni::TrayService::new(WallShuffTray);
    let _handle = service.handle();

    service.spawn();

    loop {
        let sleep_duration = duration_until_midnight();
        std::thread::sleep(sleep_duration);

        if let Err(e) = wall_shuff::run_shuffle() {
            eprintln!("Error during scheduled midnight shuffle: {}", e);
        }
    }
}

#[cfg(not(target_os = "linux"))]
fn main() {
    eprintln!("wall_shuffd is only supported on Linux.");
}
