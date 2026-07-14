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
    #[cfg(target_os = "windows")]
    let (detected_de, is_major_de) = ("Win32 API".to_string(), true);

    #[cfg(not(target_os = "windows"))]
    let (detected_de, is_major_de) = {
        let desktop = env::var("XDG_CURRENT_DESKTOP")
            .unwrap_or_else(|_| "unknown".to_string())
            .to_lowercase();

        let is_de =
            desktop.contains("gnome") || desktop.contains("kde") || desktop.contains("plasma");
        let de_name = if desktop.contains("kde") || desktop.contains("plasma") {
            "kde".to_string()
        } else if desktop.contains("gnome") {
            "gnome".to_string()
        } else {
            "N/A".to_string()
        };

        (de_name, is_de)
    };

    if config_path.exists() {
        if let Ok(content) = fs::read_to_string(config_path) {
            if let Ok(config) = toml::from_str::<AppConfig>(&content) {
                return config;
            }
        }
    }

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
            "2" => "swaybg".to_string(),
            "3" => "awww".to_string(),
            "1" => "dms".to_string(),
            "4" => "noctalia".to_string(),
            _ => "awww".to_string(),
        }
    };

    let mut default_weights = HashMap::new();
    let mut weights_toml_buffer = String::new();

    for path in available_genres {
        if let Some(filename) = path.file_name() {
            let dir_name = filename.to_string_lossy().into_owned();
            default_weights.insert(dir_name.clone(), 1.0);
            weights_toml_buffer.push_str(&format!("\"{}\" = 1.0\n", dir_name));
        }
    }

    let new_config = AppConfig {
        de_backend: detected_de,
        wc_daemon: selected_daemon,
        history_limit: 7,
        weights: default_weights,
        kde: KdeConfig {
            lockscreen_support: true,
        },
    };

    let toml_string = format!(
        "# Wallpaper Shuffler Configuration\n\n\
         # Automatically detected desktop environment (e.g. \"Win32 API\", \"kde\", \"gnome\", \"N/A\")\n\
         de_backend = \"{}\"\n\n\
         # Daemon target for wayland compositors. Ignored if a major DE is active.\n\
         # Supported choices: \"dms\", \"noctalia\", \"swaybg\", \"awww\", \"N/A\"\n\
         wc_daemon = \"{}\"\n\n\
         history_limit = {}\n\n\
         [kde]\n\
         # Sync the lock screen background with your desktop background\n\
         lockscreen_support = true\n\n\
         [weights]\n\
         # Discovered background folders. Increase the value to make a choice more frequent.\n\
         # A weight of 0 will completely disable a folder from rolling.\n\
         {}",
        new_config.de_backend, new_config.wc_daemon, new_config.history_limit, weights_toml_buffer
    );

    if let Some(parent) = config_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    if fs::write(config_path, toml_string).is_ok() && !is_major_de {
        println!("Config successfully written to: {:?}", config_path);
    }

    new_config
}
