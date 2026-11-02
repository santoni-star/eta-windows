use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentState {
    pub status: String,
    pub current_thought: Option<String>,
    pub current_action: Option<String>,
    pub history: Vec<crate::agent::AgentEvent>,
    pub context: crate::agent::AgentContext,
}
