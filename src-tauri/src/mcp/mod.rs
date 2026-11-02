pub mod commands;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerConfig {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub env: std::collections::HashMap<String, String>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpTool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
    pub server: String,
}

pub mod commands {
    use tauri::State;
    use crate::AppState;

    #[tauri::command]
    pub async fn list_servers(
        state: State<'_, AppState>,
    ) -> Result<Vec<McpServerConfig>, String> {
        let config = state.config.lock().unwrap();
        // MCP servers would be stored in config
        Ok(vec![])
    }

    #[tauri::command]
    pub async fn connect_server(
        name: String,
        _state: State<'_, AppState>,
    ) -> Result<(), String> {
        // Connect to MCP server
        Ok(())
    }
}
