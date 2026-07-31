use notify_rust::Notification;
use std::error::Error;

fn show_reshuffle_notification() -> Result<(), Box<dyn Error>> {
    // 1. Run the shuffle logic using our shared lib!
    wall_shuff::run_shuffle()?;

    // 2. Spawn the notification WITH the interactive action button
    // Note: Since wall_shuffd is a long-running daemon, calling wait_for_action()
    // here won't hang systemd or block CLI runs!
    let handle = Notification::new()
        .summary("Wallpaper Updated")
        .body("Click reshuffle to pick another wallpaper")
        .appname("Wall Shuff")
        .icon("media-playlist-shuffle")
        .action("reshuffle", "Reshuffle")
        .timeout(5000)
        .show()?;

    // Wait for the action in this thread. When clicked, it recursively triggers a new shuffle!
    handle.wait_for_action(|action| {
        if action == "reshuffle" {
            // Trigger the next notification & shuffle cycle
            let _ = show_reshuffle_notification();
        }
    });

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting wall_shuffd daemon...");

    // Optionally run once on daemon startup
    if let Err(e) = show_reshuffle_notification() {
        eprintln!("Error showing initial notification: {}", e);
    }

    // Keep the main thread alive to ensure systemd sees Type=simple as running
    loop {
        std::thread::sleep(std::time::Duration::from_secs(3600));
    }
}
