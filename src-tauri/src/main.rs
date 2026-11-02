#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod agent;
mod config;
mod mcp;
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
use system::{GlobalHotkey, SystemTrayManager};

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

    let app_state_clone = app_state.clone();

    tauri::Builder::default()
        .manage(app_state)
        .system_tray(SystemTray::new().with_id("main-tray"))
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                let id_str = id.to_string();
                match id_str.as_str() {
                    "show" => {
                        let window = app.get_window("main").unwrap();
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                    "hide" => {
                        let window = app.get_window("main").unwrap();
                        window.hide().unwrap();
                    }
                    "quit" => {
                        std::process::exit(0);
                    }
                    _ => {}
                }
            }
            SystemTrayEvent::DoubleClick { .. } => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
                window.set_focus().unwrap();
            }
            _ => {}
        })
        .on_window_event(|event| match event.event() {
            WindowEvent::CloseRequested { window, .. } => {
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
            tools::commands::list_files,
            tools::commands::read_file,
            tools::commands::write_file,
            tools::commands::execute_command,
            tools::commands::get_terminal_output,
            tools::commands::click_at,
            tools::commands::type_text,
            tools::commands::press_key,
            tools::commands::screenshot,
            tools::commands::list_windows,
            tools::commands::focus_window,
            voice::commands::start_listening,
            voice::commands::stop_listening,
            voice::commands::speak,
            system::commands::get_hotkey,
            system::commands::set_hotkey,
            config::commands::get_config,
            config::commands::save_config,
            mcp::commands::list_servers,
            mcp::commands::connect_server,
            providers::commands::list_providers,
            providers::commands::test_provider,
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();
            let app_state = app.state::<AppState>();

            let hotkey = app_state.config.lock().unwrap().global_hotkey.clone();
            let app_state_inner = app_state.clone();
            std::thread::spawn(move || {
                if let Err(e) = GlobalHotkey::listen(hotkey, move || {
                    let handle = app_handle.clone();
                    tauri::async_runtime::spawn(async move {
                        let window = handle.get_window("main");
                        if let Some(w) = window {
                            if w.is_visible().unwrap_or(false) {
                                let _ = w.hide();
                            } else {
                                let _ = w.show();
                                let _ = w.set_focus();
                            }
                        }
                    });
                }) {
                    tracing::error!("Failed to start global hotkey: {}", e);
                }
            });

            let _tray = SystemTrayManager::new(app.handle());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
