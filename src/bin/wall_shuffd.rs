#[cfg(target_os = "linux")]
use std::error::Error;
#[cfg(target_os = "linux")]
use std::sync::{Arc, Mutex};
#[cfg(target_os = "linux")]
use std::time::{Duration, Instant};

#[cfg(target_os = "linux")]
#[derive(Clone, PartialEq, Debug)]
enum RotationInterval {
    Hourly,
    Daily,
    Weekly,
    Never,
}

#[cfg(target_os = "linux")]
struct WallShuffTray {
    selected_interval: Arc<Mutex<RotationInterval>>,
}

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

        let current = self.selected_interval.lock().unwrap().clone();
        let selected_interval = Arc::clone(&self.selected_interval);

        let make_interval_item = |label: &'static str, target: RotationInterval| {
            let selected = current == target;
            let interval_store = Arc::clone(&selected_interval);

            StandardItem {
                label: if selected {
                    format!("✓ {}", label)
                } else {
                    format!("   {}", label)
                },
                activate: Box::new(move |_| {
                    if let Ok(mut lock) = interval_store.lock() {
                        *lock = target.clone();
                    }
                }),
                ..Default::default()
            }
        };

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
            SubMenu {
                label: "Rotation Frequency".into(),
                submenu: vec![
                    make_interval_item("Every Hour", RotationInterval::Hourly).into(),
                    make_interval_item("Daily (00:00:00)", RotationInterval::Daily).into(),
                    make_interval_item("Weekly", RotationInterval::Weekly).into(),
                    MenuItem::Separator,
                    make_interval_item("Disabled / Manual Only", RotationInterval::Never).into(),
                ],
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
fn get_target_duration(interval: &RotationInterval) -> Duration {
    use chrono::Local;

    match interval {
        RotationInterval::Hourly => Duration::from_secs(3600),
        RotationInterval::Daily => {
            let now = Local::now();
            let tomorrow = (now.date_naive() + chrono::Days::new(1))
                .and_hms_opt(0, 0, 0)
                .unwrap();
            let duration = tomorrow - now.naive_local();
            Duration::from_secs(duration.num_seconds().max(1) as u64)
        }
        RotationInterval::Weekly => Duration::from_secs(7 * 24 * 3600),
        RotationInterval::Never => Duration::from_secs(u64::MAX),
    }
}

#[cfg(target_os = "linux")]
fn main() -> Result<(), Box<dyn Error>> {
    let current_interval = Arc::new(Mutex::new(RotationInterval::Daily)); // Default to Hourly for testing

    let tray = WallShuffTray {
        selected_interval: Arc::clone(&current_interval),
    };

    let service = ksni::TrayService::new(tray);
    let _handle = service.handle();
    service.spawn();

    let mut last_interval = current_interval.lock().unwrap().clone();
    let mut timer_start = Instant::now();

    loop {
        std::thread::sleep(Duration::from_secs(1));

        let active_preset = current_interval.lock().unwrap().clone();

        // Reset the countdown timer instantly if the user changes the mode in the tray
        if active_preset != last_interval {
            last_interval = active_preset.clone();
            timer_start = Instant::now();
            println!("Interval changed to {:?}", active_preset);
        }

        if active_preset == RotationInterval::Never {
            continue;
        }

        let target_duration = get_target_duration(&active_preset);

        if timer_start.elapsed() >= target_duration {
            timer_start = Instant::now(); // Reset timer before running

            if let Err(e) = wall_shuff::run_shuffle() {
                eprintln!("Error during scheduled shuffle: {}", e);
            }
        }
    }
}

#[cfg(not(target_os = "linux"))]
fn main() {
    eprintln!("wall_shuffd is only supported on Linux.");
}
