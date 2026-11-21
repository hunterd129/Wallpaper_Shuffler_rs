use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub de_backend: String,
    pub wc_daemon: String,
    pub history_limit: usize,

    #[serde(default)]
    pub weights: HashMap<String, f64>,

    #[serde(default)]
    pub kde: KdeConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KdeConfig {
    #[serde(default = "default_true")]
    pub lockscreen_support: bool,
}

fn default_true() -> bool {
    true
}

impl Default for KdeConfig {
    fn default() -> Self {
        Self {
            lockscreen_support: true,
        }
    }
}

pub fn load_or_create_config(config_path: &Path, available_genres: &[PathBuf]) -> AppConfig {
    let desktop = env::var("XDG_CURRENT_DESKTOP")
        .unwrap_or_else(|_| "unknown".to_string())
        .to_lowercase();

    let is_major_de =
        desktop.contains("gnome") || desktop.contains("kde") || desktop.contains("plasma");
    let is_target_compositor =
        desktop.contains("niri") || desktop.contains("hyprland") || desktop.contains("mangowm");
    let detected_de = "auto".to_string();

    let mut existing_config = None;
    let mut needs_setup = false;

    if config_path.exists() {
        if let Ok(content) = fs::read_to_string(config_path) {
            if let Ok(config) = toml::from_str::<AppConfig>(&content) {
                if config.wc_daemon.eq_ignore_ascii_case("n/a") && is_target_compositor {
                    needs_setup = true;
                }
                existing_config = Some(config);
            } else {
                needs_setup = true;
            }
        } else {
            needs_setup = true;
        }
    } else {
        needs_setup = true;
    }

    let mut config = if !needs_setup {
        existing_config.unwrap()
    } else {
        let selected_daemon = if is_major_de {
            "N/A".to_string()
        } else {
            println!("Please select your background controller daemon (WC Daemon):");
            println!("  [1] Dank Material Shell (dms)");
            println!("  [2] swaybg (Wayland)");
            println!("  [3] awww (Animated/Wayland)");
            println!("  [4] Noctalia");
            print!("Enter choice (1-4): ");
            io::stdout().flush().unwrap();

            let mut choice = String::new();
            io::stdin().read_line(&mut choice).unwrap();

            match choice.trim() {
                "1" => "dms".to_string(),
                "2" => "swaybg".to_string(),
                "3" => "awww".to_string(),
                "4" => "noctalia".to_string(),
                _ => "awww".to_string(),
            }
        };

        if let Some(mut existing) = existing_config {
            existing.wc_daemon = selected_daemon;
            existing
        } else {
            let mut default_weights = HashMap::new();
            for path in available_genres {
                if let Some(filename) = path.file_name() {
                    let dir_name = filename.to_string_lossy().into_owned();
                    default_weights.insert(dir_name, 1.0);
                }
            }

            AppConfig {
                de_backend: detected_de,
                wc_daemon: selected_daemon,
                history_limit: 7,
                weights: default_weights,
                kde: KdeConfig {
                    lockscreen_support: true,
                },
            }
        }
    };

    for path in available_genres {
        if let Some(filename) = path.file_name() {
            let dir_name = filename.to_string_lossy().into_owned();
            config.weights.entry(dir_name).or_insert(1.0);
        }
    }

    let mut weights_toml_buffer = String::new();
    for (dir_name, weight) in &config.weights {
        weights_toml_buffer.push_str(&format!("\"{}\" = {:.1}\n", dir_name, weight));
    }

    let toml_string = format!(
        "# Wallpaper Shuffler Configuration\n\n\
         # Desktop backend setting. Choices: \"auto\", \"gnome\", \"kde\", \"N/A\"\n\
         # \"auto\" resolves dynamically using $XDG_CURRENT_DESKTOP on Linux.\n\
         # Set to \"auto\" by default.\n\
         de_backend = \"{}\"\n\n\
         # Wallpaper Daemon for wayland compositors. Used if de_backend resolves to a WC.\n\n\
         # If wc_daemon is set to \"N/A\" and $XDG_CURRENT_DESKTOP returns one of the supported WCs,\n\
         # you will be prompted to select one of the supported options.\n\n\
         # Supported choices: \"dms\", \"noctalia\", \"swaybg\", \"awww\", \"N/A\"\n\
         wc_daemon = \"{}\"\n\n\
         history_limit = {}\n\n\
         [kde]\n\
         # Sync the lock screen background with your desktop background\n\
         # set to true by default\n\
         lockscreen_support = true\n\n\
         [weights]\n\
         # Discovered wallpaper dirs. Increase the value to make a choice more frequent.\n\
         # A weight of 0 will disable a dir from being selected.\n\
         {}",
        config.de_backend, config.wc_daemon, config.history_limit, weights_toml_buffer
    );

    if let Some(parent) = config_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    if fs::write(config_path, toml_string).is_ok() && !is_major_de && needs_setup {
        println!("Config successfully written to: {:?}", config_path);
    }

    config
}
