use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopState {
    pub state: LoopStatus,
    pub max_iterations: u32,
    pub current_iteration: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoopStatus {
    Idle,
    Thinking,
    ExecutingTool { tool: String, args: serde_json::Value },
    WaitingForUser,
    Finished,
}

pub struct LoopManager {
    pub state: LoopState,
}

impl LoopManager {
    pub fn new() -> Self {
        Self {
            state: LoopState {
                state: LoopStatus::Idle,
                max_iterations: 10,
                current_iteration: 0,
            },
        }
    }

    pub fn step(&mut self) -> Result<LoopState, Box<dyn std::error::Error>> {
        self.state.current_iteration += 1;

        if self.state.current_iteration > self.state.max_iterations {
            self.state.state = LoopStatus::Finished;
            return Ok(self.state.clone());
        }

        match &self.state.state {
            LoopStatus::Idle => {
                self.state.state = LoopStatus::Thinking;
            }
            LoopStatus::Thinking => {
                self.state.state = LoopStatus::ExecutingTool {
                    tool: "placeholder".to_string(),
                    args: serde_json::json!({}),
                };
            }
            LoopStatus::ExecutingTool { .. } => {
                self.state.state = LoopStatus::WaitingForUser;
            }
            LoopStatus::WaitingForUser => {}
            LoopStatus::Finished => {}
        }

        Ok(self.state.clone())
    }

    pub fn reset(&mut self) {
        self.state.state = LoopStatus::Idle;
        self.state.current_iteration = 0;
    }
}
