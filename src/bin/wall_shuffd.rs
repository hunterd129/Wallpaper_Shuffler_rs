#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Shared imports
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Clone, PartialEq, Debug)]
enum RotationInterval {
    Hourly,
    Daily,
    Weekly,
    Never,
}

fn get_next_trigger_time(interval: &RotationInterval) -> chrono::DateTime<chrono::Local> {
    use chrono::{Duration as ChronoDuration, Local, LocalResult, Timelike};

    let now = Local::now();

    match interval {
        RotationInterval::Hourly => {
            let top_of_current_hour = now
                .with_minute(0)
                .and_then(|dt| dt.with_second(0))
                .and_then(|dt| dt.with_nanosecond(0));

            match top_of_current_hour {
                Some(dt) => dt + ChronoDuration::hours(1),
                None => now + ChronoDuration::hours(1),
            }
        }
        RotationInterval::Daily => {
            let tomorrow = (now.date_naive() + chrono::Days::new(1))
                .and_hms_opt(0, 0, 0)
                .unwrap();

            match tomorrow.and_local_timezone(Local) {
                LocalResult::Single(dt) => dt,
                _ => now + ChronoDuration::days(1),
            }
        }
        RotationInterval::Weekly => now + ChronoDuration::weeks(1),
        RotationInterval::Never => now + ChronoDuration::weeks(5200),
    }
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
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let current_interval = Arc::new(Mutex::new(RotationInterval::Daily));

    let tray = WallShuffTray {
        selected_interval: Arc::clone(&current_interval),
    };

    let service = ksni::TrayService::new(tray);
    let _handle = service.handle();
    service.spawn();

    let mut last_interval = current_interval.lock().unwrap().clone();
    let mut next_trigger_time = get_next_trigger_time(&last_interval);

    loop {
        std::thread::sleep(Duration::from_secs(1));

        let active_preset = current_interval.lock().unwrap().clone();

        if active_preset != last_interval {
            last_interval = active_preset.clone();
            next_trigger_time = get_next_trigger_time(&active_preset);
            println!("Interval changed to {:?}", active_preset);
        }

        if active_preset == RotationInterval::Never {
            continue;
        }

        if chrono::Local::now() >= next_trigger_time {
            next_trigger_time = get_next_trigger_time(&active_preset);

            if let Err(e) = wall_shuff::run_shuffle() {
                eprintln!("Error during scheduled shuffle: {}", e);
            }
        }
    }
}

#[cfg(target_os = "windows")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use muda::{Menu, MenuItem, PredefinedMenuItem, Submenu};
    use tray_icon::{Icon, TrayIconBuilder};

    let current_interval = Arc::new(Mutex::new(RotationInterval::Daily));

    let menu = Menu::new();
    let item_shuffle = MenuItem::new("Shuffle Now", true, None);

    let interval_menu = Submenu::new("Rotation Frequency", true);
    let item_hourly = MenuItem::new("Every Hour", true, None);
    let item_daily = MenuItem::new("Daily (00:00:00)", true, None);
    let item_weekly = MenuItem::new("Weekly", true, None);
    let item_never = MenuItem::new("Disabled / Manual Only", true, None);

    let _ = interval_menu.append_items(&[
        &item_hourly,
        &item_daily,
        &item_weekly,
        &PredefinedMenuItem::separator(),
        &item_never,
    ]);

    let item_quit = MenuItem::new("Quit", true, None);

    let _ = menu.append_items(&[
        &item_shuffle,
        &PredefinedMenuItem::separator(),
        &interval_menu,
        &PredefinedMenuItem::separator(),
        &item_quit,
    ]);

    let (icon_rgba, icon_width, icon_height) = (vec![255u8; 32 * 32 * 4], 32, 32);
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height)?;

    let _tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip("Wall Shuff")
        .with_icon(icon)
        .build()?;

    let interval_clone = Arc::clone(&current_interval);
    std::thread::spawn(move || {
        let mut last_interval = interval_clone.lock().unwrap().clone();
        let mut next_trigger_time = get_next_trigger_time(&last_interval);

        loop {
            std::thread::sleep(Duration::from_secs(1));

            let active_preset = interval_clone.lock().unwrap().clone();

            if active_preset != last_interval {
                last_interval = active_preset.clone();
                next_trigger_time = get_next_trigger_time(&active_preset);
            }

            if active_preset == RotationInterval::Never {
                continue;
            }

            if chrono::Local::now() >= next_trigger_time {
                next_trigger_time = get_next_trigger_time(&active_preset);
                if let Err(e) = wall_shuff::run_shuffle() {
                    eprintln!("Error during scheduled shuffle: {}", e);
                }
            }
        }
    });

    let menu_channel = muda::MenuEvent::receiver();

    loop {
        // Dispatch Win32 messages
        unsafe {
            use windows::Win32::UI::WindowsAndMessaging::{
                DispatchMessageW, PeekMessageW, TranslateMessage, MSG, PM_REMOVE,
            };
            let mut msg = MSG::default();
            while PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).into() {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }

        // Handle Tray Menu Events
        if let Ok(event) = menu_channel.try_recv() {
            if event.id == item_shuffle.id() {
                let _ = wall_shuff::run_shuffle();
            } else if event.id == item_hourly.id() {
                *current_interval.lock().unwrap() = RotationInterval::Hourly;
            } else if event.id == item_daily.id() {
                *current_interval.lock().unwrap() = RotationInterval::Daily;
            } else if event.id == item_weekly.id() {
                *current_interval.lock().unwrap() = RotationInterval::Weekly;
            } else if event.id == item_never.id() {
                *current_interval.lock().unwrap() = RotationInterval::Never;
            } else if event.id == item_quit.id() {
                std::process::exit(0);
            }
        }

        std::thread::sleep(Duration::from_millis(16));
    }
}
