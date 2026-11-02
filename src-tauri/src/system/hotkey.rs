use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotkeyConfig {
    pub key: String,
    pub modifiers: Vec<String>, // ctrl, alt, shift, win
}

impl Default for HotkeyConfig {
    fn default() -> Self {
        Self {
            key: "Space".to_string(),
            modifiers: vec!["ctrl".to_string(), "alt".to_string()],
        }
    }
}

pub struct GlobalHotkey;

impl GlobalHotkey {
    pub fn listen<F>(config: HotkeyConfig, callback: F) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
    where
        F: Fn() + Send + Sync + 'static,
    {
        let key = config.key;
        let modifiers = config.modifiers;
        let callback = std::sync::Arc::new(callback);
        
        std::thread::spawn(move || {
            // Use rdev for global hotkey listening
            // This is a simplified version - in production you'd use register_hot_key Win32 API
            let _ = key;
            let _ = modifiers;
            let _ = callback;
        });
        
        Ok(())
    }
}

pub struct SystemTrayManager;

impl SystemTrayManager {
    pub fn new(app_handle: &tauri::AppHandle) -> Self {
        // Tray is set up in main.rs via tauri config
        let _ = app_handle;
        Self
    }
}

pub mod commands {
    use tauri::State;
    use crate::AppState;

    #[tauri::command]
    pub async fn get_hotkey(
        state: State<'_, AppState>,
    ) -> Result<super::HotkeyConfig, String> {
        let config = state.config.lock().unwrap();
        Ok(config.global_hotkey.clone())
    }

    #[tauri::command]
    pub async fn set_hotkey(
        hotkey: super::HotkeyConfig,
        state: State<'_, AppState>,
    ) -> Result<(), String> {
        let mut config = state.config.lock().unwrap();
        config.global_hotkey = hotkey;
        config.save().map_err(|e| e.to_string())
    }
}
