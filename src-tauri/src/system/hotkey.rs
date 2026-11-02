use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotkeyConfig {
    pub key: String,
    pub modifiers: Vec<String>,
}

impl Default for HotkeyConfig {
    fn default() -> Self {
        Self {
            key: "Space".to_string(),
            modifiers: vec!["ctrl".to_string(), "alt".to_string()],
        }
    }
}

pub mod commands {
    use tauri::State;
    use crate::AppState;

    #[tauri::command]
    pub async fn get_hotkey(
        state: State<'_, AppState>,
    ) -> Result<crate::system::hotkey::HotkeyConfig, String> {
        let config = state.config.lock().unwrap();
        Ok(config.global_hotkey.clone())
    }

    #[tauri::command]
    pub async fn set_hotkey(
        hotkey: crate::system::hotkey::HotkeyConfig,
        state: State<'_, AppState>,
    ) -> Result<(), String> {
        let mut config = state.config.lock().unwrap();
        config.global_hotkey = hotkey;
        config.save().map_err(|e| e.to_string())
    }
}
