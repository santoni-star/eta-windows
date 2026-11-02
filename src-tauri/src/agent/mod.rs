pub mod commands;
pub mod loop_manager;
pub mod memory;
pub mod state;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentStatus {
    Idle,
    Thinking,
    Executing,
    WaitingForInput,
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentState {
    pub status: AgentStatus,
    pub current_thought: Option<String>,
    pub current_action: Option<String>,
    pub history: Vec<AgentEvent>,
    pub context: AgentContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentEvent {
    UserMessage { content: String, timestamp: chrono::DateTime<chrono::Utc> },
    AssistantMessage { content: String, timestamp: chrono::DateTime<chrono::Utc> },
    ToolCall { name: String, args: serde_json::Value, timestamp: chrono::DateTime<chrono::Utc> },
    ToolResult { name: String, result: String, success: bool, timestamp: chrono::DateTime<chrono::Utc> },
    Thought { content: String, timestamp: chrono::DateTime<chrono::Utc> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentContext {
    pub working_directory: PathBuf,
    pub active_window: Option<String>,
    pub clipboard: Option<String>,
    pub screenshot: Option<String>,
    pub environment_variables: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Agent {
    pub state: AgentState,
    pub data_dir: PathBuf,
    pub memory: memory::MemoryStore,
}

impl Agent {
    pub fn new(data_dir: PathBuf) -> Result<Self> {
        let memory = memory::MemoryStore::new(data_dir.join("memory.db"))?;
        
        Ok(Self {
            state: AgentState {
                status: AgentStatus::Idle,
                current_thought: None,
                current_action: None,
                history: Vec::new(),
                context: AgentContext {
                    working_directory: std::env::current_dir().unwrap_or(PathBuf::from("C:\\")),
                    active_window: None,
                    clipboard: None,
                    screenshot: None,
                    environment_variables: std::env::vars().collect(),
                },
            },
            data_dir,
            memory,
        })
    }

    pub async fn process_message(&mut self, message: String) -> Result<String> {
        // Add user message to history
        self.history.push(AgentEvent::UserMessage {
            content: message.clone(),
            timestamp: chrono::Utc::now(),
        });

        self.state.status = AgentStatus::Thinking;
        self.state.current_thought = Some(format!("Processing: {}", message));

        // Save to memory
        self.memory.store_interaction(&message, None).await?;

        // Generate response using provider
        let response = self.generate_response(&message).await?;

        // Add assistant message
        self.history.push(AgentEvent::AssistantMessage {
            content: response.clone(),
            timestamp: chrono::Utc::now(),
        });

        self.memory.store_interaction(&message, Some(&response)).await?;

        self.state.status = AgentStatus::Idle;
        self.state.current_thought = None;

        Ok(response)
    }

    async fn generate_response(&mut self, message: &str) -> Result<String> {
        // This would integrate with the provider system
        // For now, return a placeholder
        Ok(format!("Я розумів ваш запит: \"{}\". Я можу допомогти з файлами, терміналом, керуванням вікнами, веб-пошуком та іншим.", message))
    }

    pub fn add_tool_result(&mut self, name: &str, result: &str, success: bool) {
        self.history.push(AgentEvent::ToolResult {
            name: name.to_string(),
            result: result.to_string(),
            success,
            timestamp: chrono::Utc::now(),
        });
    }

    pub fn add_tool_call(&mut self, name: &str, args: serde_json::Value) {
        self.history.push(AgentEvent::ToolCall {
            name: name.to_string(),
            args,
            timestamp: chrono::Utc::now(),
        });
    }
}

pub mod commands {
    use tauri::State;

    use crate::AppState;
    use crate::agent::AgentState;

    #[tauri::command]
    pub async fn send_message(
        message: String,
        state: State<'_, AppState>,
    ) -> Result<String, String> {
        let mut agent = state.agent.lock().unwrap();
        agent.process_message(message)
            .await
            .map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub async fn get_history(
        state: State<'_, AppState>,
    ) -> Result<Vec<crate::agent::AgentEvent>, String> {
        let agent = state.agent.lock().unwrap();
        Ok(agent.state.history.clone())
    }

    #[tauri::command]
    pub async fn clear_history(
        state: State<'_, AppState>,
    ) -> Result<(), String> {
        let mut agent = state.agent.lock().unwrap();
        agent.state.history.clear();
        Ok(())
    }

    #[tauri::command]
    pub async fn get_state(
        state: State<'_, AppState>,
    ) -> Result<AgentState, String> {
        let agent = state.agent.lock().unwrap();
        Ok(agent.state.clone())
    }
}
