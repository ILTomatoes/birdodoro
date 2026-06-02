use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppConfig {
    pub work_duration: u32,
    pub short_break_duration: u32,
    pub long_break_duration: u32,
    pub pomodoros_per_cycle: u32,
    pub animation_enabled: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            work_duration: 25,
            short_break_duration: 5,
            long_break_duration: 15,
            pomodoros_per_cycle: 4,
            animation_enabled: true,
        }
    }
}

impl AppConfig {
    fn config_path(app: &tauri::AppHandle) -> PathBuf {
        app.path().app_data_dir().unwrap_or_else(|_| PathBuf::from(".")).join("config.json")
    }

    pub fn load(app: &tauri::AppHandle) -> Self {
        let path = Self::config_path(app);
        if path.exists() {
            match fs::read_to_string(&path) {
                Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
                Err(_) => Self::default(),
            }
        } else {
            Self::default()
        }
    }

    pub fn save(&self, app: &tauri::AppHandle) -> Result<(), String> {
        let path = Self::config_path(app);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let json = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(&path, json).map_err(|e| e.to_string())
    }
}
