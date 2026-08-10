pub mod config;
pub mod os;

use rand::distributions::{Distribution, WeightedIndex};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Serialize, Deserialize, Default)]
struct History {
    recent: Vec<PathBuf>,
}

fn get_wallpaper_genres_list(root_path: &Path) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let genres: Vec<PathBuf> = fs::read_dir(root_path)?
        .filter_map(|res| res.ok())
        .map(|e| e.path())
        .filter(|path| path.is_dir())
        .collect();

    if genres.is_empty() {
        return Err("No directories found in ~/Pictures/Wallpapers".into());
    }
    Ok(genres)
}

pub fn run_shuffle() -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir().ok_or("Could not find Home")?;
    let pictures_dir = dirs::picture_dir().ok_or("Could not find Pictures")?;
    let root_path = pictures_dir.join("Wallpapers");

    let genres = get_wallpaper_genres_list(&root_path)?;

    let config_path = home_dir
        .join(".config")
        .join("wall_shuff")
        .join("config.toml");

    let app_config = config::load_or_create_config(&config_path, &genres);

    let history_root = home_dir.join(".local").join("share").join("wall_shuff");
    let history_path = history_root.join("history.toml");

    if !history_root.exists() {
        fs::create_dir_all(&history_root)?;
    }

    let mut rng = rand::thread_rng();
    let mut weights = Vec::new();

    for genre_path in &genres {
        let dir_name = genre_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();

        let weight = app_config.weights.get(&dir_name).cloned().unwrap_or(1.0);
        weights.push(weight);
    }

    let total_weight: f64 = weights.iter().sum();

    let genre = if total_weight > 0.0 {
        let dist = WeightedIndex::new(&weights)?;
        &genres[dist.sample(&mut rng)]
    } else {
        genres
            .choose(&mut rng)
            .ok_or("Genres vector was unexpectedly empty")?
    };

    let entries: Vec<PathBuf> = WalkDir::new(genre)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .map(|e| e.path().to_owned())
        .collect();

    if entries.is_empty() {
        return Err("No images found in the chosen genre".into());
    }

    let mut history: History = fs::read_to_string(&history_path)
        .ok()
        .and_then(|content| toml::from_str(&content).ok())
        .unwrap_or_default();

    let fresh_options: Vec<PathBuf> = entries
        .iter()
        .filter(|p| !history.recent.contains(p))
        .cloned()
        .collect();

    let wall = if !fresh_options.is_empty() {
        fresh_options.choose(&mut rng).unwrap().clone()
    } else {
        history.recent.clear();
        entries.choose(&mut rng).unwrap().clone()
    };

    if history.recent.len() >= app_config.history_limit {
        history.recent.remove(0);
    }
    history.recent.push(wall.clone());
    fs::write(&history_path, toml::to_string_pretty(&history)?)?;

    os::set_wallpaper(&wall, &app_config)?;

    Ok(())
}
