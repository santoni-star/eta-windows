use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceConfig {
    pub enabled: bool,
    pub stt_provider: String,
    pub tts_provider: String,
    pub language: String,
    pub voice_id: Option<String>,
}

impl Default for VoiceConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            stt_provider: "whisper".to_string(),
            tts_provider: "edge".to_string(),
            language: "uk-UA".to_string(),
            voice_id: None,
        }
    }
}

pub mod commands {
    use tauri::State;
    use crate::AppState;

    #[tauri::command]
    pub async fn start_listening(
        _state: State<'_, AppState>,
    ) -> Result<(), String> {
        Ok(())
    }

    #[tauri::command]
    pub async fn stop_listening(
        _state: State<'_, AppState>,
    ) -> Result<(), String> {
        Ok(())
    }

    #[tauri::command]
    pub async fn speak(
        text: String,
        _state: State<'_, AppState>,
    ) -> Result<(), String> {
        Ok(())
    }
}
