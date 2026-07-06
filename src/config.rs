use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub wm_backend: String,
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

    if config_path.exists() {
        if let Ok(content) = fs::read_to_string(config_path) {
            if let Ok(config) = toml::from_str::<AppConfig>(&content) {
                return config;
            }
        }
    }

    let selected_backend = if is_major_de {
        "native".to_string()
    } else {
        println!("Please select your background controller:");
        println!("  [1] Dank Material Shell (dms)");
        println!("  [2] swaybg (Wayland)");
        println!("  [3] swww (Animated/Wayland)");
        print!("Enter choice (1-3): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "2" => "swaybg".to_string(),
            "3" => "swww".to_string(),
            "1" => "dms".to_string(),
            _ => "dms".to_string(),
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
        wm_backend: selected_backend,
        history_limit: 7,
        weights: default_weights,
        kde: KdeConfig {
            lockscreen_support: true,
        },
    };

    let toml_string = format!(
        "# Wallpaper Shuffler Configuration\n\n\
         # Note: wm_backend only applies to custom Window Managers (Hyprland, Niri, MangoWM, etc.)\n\
         # Supported choices: \"dms\", \"swaybg\", \"swww\"\n\
         wm_backend = \"{}\"\n\
         history_limit = {}\n\n\
         [kde]\n\
         # Automatically sync the login/lock screen background with your desktop wallpaper\n\
         lockscreen_support = true\n\n\
         [weights]\n\
         # Discovered background folders. Increase the value to make a choice more frequent.\n\
         # A weight of 0 will completely disable a folder from rolling.\n\
         {}",
        new_config.wm_backend, new_config.history_limit, weights_toml_buffer
    );

    if let Some(parent) = config_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    if fs::write(config_path, toml_string).is_ok() && !is_major_de {
        println!("Config successfully written to: {:?}", config_path);
    }

    new_config
}
