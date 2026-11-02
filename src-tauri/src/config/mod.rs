use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub global_hotkey: crate::system::hotkey::HotkeyConfig,
    pub providers: Vec<crate::providers::ProviderConfig>,
    pub voice: crate::voice::VoiceConfig,
    pub theme: String,
    pub language: String,
    pub auto_start: bool,
    pub minimize_to_tray: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            global_hotkey: crate::system::hotkey::HotkeyConfig::default(),
            providers: vec![],
            voice: crate::voice::VoiceConfig::default(),
            theme: "system".to_string(),
            language: "uk".to_string(),
            auto_start: false,
            minimize_to_tray: true,
        }
    }
}

impl AppConfig {
    pub fn load(config_dir: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = config_dir.join("config.yaml");

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: Self = serde_yaml::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_dir = directories::ProjectDirs::from("io", "github", "mangi.eta")
            .ok_or("Failed to get project dirs")?;
        let config_path = config_dir.config_dir().join("config.yaml");

        let content = serde_yaml::to_string(self)?;
        std::fs::write(&config_path, content)?;

        Ok(())
    }
}

pub mod commands {
    use tauri::State;
    use crate::AppState;

    #[tauri::command]
    pub async fn get_config(
        state: State<'_, AppState>,
    ) -> Result<crate::config::AppConfig, String> {
        let config = state.config.lock().unwrap();
        Ok(config.clone())
    }

    #[tauri::command]
    pub async fn save_config(
        config: crate::config::AppConfig,
        state: State<'_, AppState>,
    ) -> Result<(), String> {
        let mut state_config = state.config.lock().unwrap();
        *state_config = config;
        state_config.save().map_err(|e| e.to_string())
    }
}
