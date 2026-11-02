pub mod commands;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub name: String,
    pub api_key: Option<String>,
    pub endpoint: Option<String>,
    pub model: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProviderType {
    OpenAI,
    Anthropic,
    GoogleGemini,
    Codex,
    Grok,
    Ollama,
    Custom,
}

impl ProviderType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "openai" => ProviderType::OpenAI,
            "anthropic" => ProviderType::Anthropic,
            "google" | "gemini" => ProviderType::GoogleGemini,
            "codex" => ProviderType::Codex,
            "grok" => ProviderType::Grok,
            "ollama" => ProviderType::Ollama,
            _ => ProviderType::Custom,
        }
    }
}

pub mod commands {
    use tauri::State;

    use crate::AppState;
    use crate::providers::ProviderConfig;

    #[tauri::command]
    pub async fn list_providers(
        state: State<'_, AppState>,
    ) -> Result<Vec<ProviderConfig>, String> {
        let config = state.config.lock().unwrap();
        Ok(config.providers.clone())
    }

    #[tauri::command]
    pub async fn test_provider(
        provider_name: String,
        state: State<'_, AppState>,
    ) -> Result<bool, String> {
        let config = state.config.lock().unwrap();
        let provider = config.providers.iter().find(|p| p.name == provider_name);
        match provider {
            Some(p) => Ok(p.enabled && p.api_key.is_some()),
            None => Ok(false),
        }
    }
}
