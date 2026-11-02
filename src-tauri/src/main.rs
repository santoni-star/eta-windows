#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod agent;
mod config;
mod providers;
mod system;
mod tools;
mod voice;

use anyhow::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{Manager, SystemTray, SystemTrayEvent, WindowEvent};
use tokio::sync::Mutex;

use agent::{Agent, AgentState};
use config::AppConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub agent: Arc<Mutex<Agent>>,
    pub config: Arc<Mutex<AppConfig>>,
    pub project_dirs: ProjectDirs,
}

impl AppState {
    pub fn new() -> Result<Self> {
        let project_dirs = ProjectDirs::from("io", "github", "mangi.eta")
            .ok_or_else(|| anyhow::anyhow!("Failed to get project directories"))?;

        std::fs::create_dir_all(project_dirs.config_dir())?;
        std::fs::create_dir_all(project_dirs.data_dir())?;
        std::fs::create_dir_all(project_dirs.cache_dir())?;

        let config = AppConfig::load(project_dirs.config_dir())?;
        let agent = Agent::new(project_dirs.data_dir().to_path_buf())?;

        Ok(Self {
            agent: Arc::new(Mutex::new(agent)),
            config: Arc::new(Mutex::new(config)),
            project_dirs,
        })
    }
}

fn main() {
    tracing_subscriber::fmt::init();

    let app_state = match AppState::new() {
        Ok(state) => state,
        Err(e) => {
            eprintln!("Failed to initialize app: {}", e);
            std::process::exit(1);
        }
    };

    tauri::Builder::default()
        .manage(app_state)
        .system_tray(SystemTray::new().with_id("main-tray"))
        .on_system_tray_event(|_app, _event| {})
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { window, .. } => {
                window.hide().unwrap();
                event.prevent_close();
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            agent::commands::send_message,
            agent::commands::get_history,
            agent::commands::clear_history,
            agent::commands::get_state,
            config::commands::get_config,
            config::commands::save_config,
        ])
        .setup(|app| {
            let _app_handle = app.handle().clone();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
