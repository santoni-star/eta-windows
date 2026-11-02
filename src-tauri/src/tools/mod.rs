use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub path: String,
    pub name: String,
    pub is_dir: bool,
    pub size: u64,
    pub modified: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowInfo {
    pub handle: u64,
    pub title: String,
    pub class_name: String,
    pub process_id: u32,
    pub is_active: bool,
    pub rect: Rect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

pub mod commands {
    use tauri::State;
    use crate::AppState;

    #[tauri::command]
    pub async fn list_files(
        path: String,
        _state: State<'_, AppState>,
    ) -> Result<Vec<FileInfo>, String> {
        let path = std::path::Path::new(&path);
        let mut entries = Vec::new();

        if let Ok(read_dir) = std::fs::read_dir(path) {
            for entry in read_dir.flatten() {
                let metadata = entry.metadata().unwrap_or_default();
                entries.push(FileInfo {
                    path: entry.path().to_string_lossy().to_string(),
                    name: entry.file_name().to_string_lossy().to_string(),
                    is_dir: metadata.is_dir(),
                    size: metadata.len(),
                    modified: metadata.modified().ok().map(|t| {
                        let dt: chrono::DateTime<chrono::Utc> = t.into();
                        dt.to_rfc3339()
                    }),
                });
            }
        }

        entries.sort_by(|a, b| b.is_dir.cmp(&a.is_dir).then_with(|| a.name.cmp(&b.name)));
        Ok(entries)
    }

    #[tauri::command]
    pub async fn read_file(
        path: String,
        _state: State<'_, AppState>,
    ) -> Result<String, String> {
        std::fs::read_to_string(&path).map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub async fn write_file(
        path: String,
        content: String,
        _state: State<'_, AppState>,
    ) -> Result<(), String> {
        std::fs::write(&path, content).map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub async fn execute_command(
        command: String,
        working_dir: Option<String>,
        _state: State<'_, AppState>,
    ) -> Result<String, String> {
        let mut cmd = std::process::Command::new("cmd");
        cmd.arg("/C").arg(command);
        if let Some(dir) = working_dir {
            cmd.current_dir(dir);
        }
        let output = cmd.output().map_err(|e| e.to_string())?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if output.status.success() {
            Ok(stdout)
        } else {
            Err(format!("{}\n{}", stdout, stderr))
        }
    }

    #[tauri::command]
    pub async fn list_windows(
        _state: State<'_, AppState>,
    ) -> Result<Vec<WindowInfo>, String> {
        Ok(vec![])
    }

    #[tauri::command]
    pub async fn focus_window(
        _handle: u64,
        _state: State<'_, AppState>,
    ) -> Result<(), String> {
        Ok(())
    }
}
