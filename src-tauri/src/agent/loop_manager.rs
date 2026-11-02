use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoopState {
    Idle,
    Thinking,
    ExecutingTool { tool: String, args: serde_json::Value },
    WaitingForUser,
    Finished,
}

pub struct LoopManager {
    pub state: LoopState,
    pub max_iterations: u32,
    pub current_iteration: u32,
}

impl LoopManager {
    pub fn new() -> Self {
        Self {
            state: LoopState::Idle,
            max_iterations: 10,
            current_iteration: 0,
        }
    }

    pub fn step(&mut self) -> Result<LoopState, Box<dyn std::error::Error>> {
        self.current_iteration += 1;
        
        if self.current_iteration > self.max_iterations {
            self.state = LoopState::Finished;
            return Ok(self.state.clone());
        }
        
        match &self.state {
            LoopState::Idle => {
                self.state = LoopState::Thinking;
            }
            LoopState::Thinking => {
                // Decide which tool to call
                self.state = LoopState::ExecutingTool {
                    tool: "placeholder".to_string(),
                    args: serde_json::json!({}),
                };
            }
            LoopState::ExecutingTool { .. } => {
                self.state = LoopState::WaitingForUser;
            }
            LoopState::WaitingForUser => {
                // Wait for user input
            }
            LoopState::Finished => {}
        }
        
        Ok(self.state.clone())
    }

    pub fn reset(&mut self) {
        self.state = LoopState::Idle;
        self.current_iteration = 0;
    }
}
