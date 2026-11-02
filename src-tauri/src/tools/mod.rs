use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub path: String,
    pub name: String,
    pub is_dir: bool,
    pub size: u64,
    pub modified: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalSession {
    pub id: String,
    pub working_directory: String,
    pub output: String,
    pub is_active: bool,
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
    pub async fn get_terminal_output(
        session_id: String,
        _state: State<'_, AppState>,
    ) -> Result<String, String> {
        // This would interface with a persistent terminal session
        Ok(format!("Session {} output would be retrieved here", session_id))
    }

    #[tauri::command]
    pub async fn click_at(
        x: i32,
        y: i32,
        _state: State<'_, AppState>,
    ) -> Result<(), String> {
        // Use Win32 API to simulate mouse click
        unsafe {
            use winapi::um::winuser::{mouse_event, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP};
            mouse_event(MOUSEEVENTF_LEFTDOWN, x as u32, y as u32, 0, 0);
            mouse_event(MOUSEEVENTF_LEFTUP, x as u32, y as u32, 0, 0);
        }
        Ok(())
    }

    #[tauri::command]
    pub async fn type_text(
        text: String,
        _state: State<'_, AppState>,
    ) -> Result<(), String> {
        // Use Win32 API SendInput to type text
        for ch in text.chars() {
            unsafe {
                use winapi::um::winuser::{keybd_event, MapVirtualKeyA, KEYEVENTF_KEYUP};
                let vk = MapVirtualKeyA(ch as u32, 0);
                keybd_event(vk as u8, 0, 0, 0);
                keybd_event(vk as u8, 0, KEYEVENTF_KEYUP, 0);
            }
        }
        Ok(())
    }

    #[tauri::command]
    pub async fn press_key(
        key: String,
        _state: State<'_, AppState>,
    ) -> Result<(), String> {
        // Map key name to virtual key code
        let vk = match key.as_str() {
            "enter" => 0x0D,
            "tab" => 0x09,
            "esc" | "escape" => 0x1B,
            "space" => 0x20,
            "backspace" => 0x08,
            "delete" => 0x2E,
            "up" => 0x26,
            "down" => 0x28,
            "left" => 0x25,
            "right" => 0x27,
            "home" => 0x24,
            "end" => 0x23,
            "pageup" => 0x21,
            "pagedown" => 0x22,
            "f1" => 0x70, "f2" => 0x71, "f3" => 0x72, "f4" => 0x73,
            "f5" => 0x74, "f6" => 0x75, "f7" => 0x76, "f8" => 0x77,
            "f9" => 0x78, "f10" => 0x79, "f11" => 0x7A, "f12" => 0x7B,
            "ctrl" => 0x11,
            "alt" => 0x12,
            "shift" => 0x10,
            "win" => 0x5B,
            "a" => 0x41, "b" => 0x42, "c" => 0x43, "d" => 0x44, "e" => 0x45,
            "f" => 0x46, "g" => 0x47, "h" => 0x48, "i" => 0x49, "j" => 0x4A,
            "k" => 0x4B, "l" => 0x4C, "m" => 0x4D, "n" => 0x4E, "o" => 0x4F,
            "p" => 0x50, "q" => 0x51, "r" => 0x52, "s" => 0x53, "t" => 0x54,
            "u" => 0x55, "v" => 0x56, "w" => 0x57, "x" => 0x58, "y" => 0x59,
            "z" => 0x5A,
            "0" => 0x30, "1" => 0x31, "2" => 0x32, "3" => 0x33, "4" => 0x34,
            "5" => 0x35, "6" => 0x36, "7" => 0x37, "8" => 0x38, "9" => 0x39,
            _ => return Err(format!("Unknown key: {}", key)),
        };
        
        unsafe {
            use winapi::um::winuser::{keybd_event, KEYEVENTF_KEYUP};
            keybd_event(vk as u8, 0, 0, 0);
            keybd_event(vk as u8, 0, KEYEVENTF_KEYUP, 0);
        }
        
        Ok(())
    }

    #[tauri::command]
    pub async fn screenshot(
        _state: State<'_, AppState>,
    ) -> Result<String, String> {
        // Capture screenshot using Win32 API
        unsafe {
            use winapi::um::winuser::{GetDC, GetSystemMetrics, ReleaseDC, SM_CXSCREEN, SM_CYSCREEN};
            use winapi::um::wingdi::{CreateCompatibleBitmap, CreateCompatibleDC, SelectObject, BitBlt, SRCCOPY, DeleteDC, DeleteObject, GetBitmapBits, BITMAP};
            use winapi::shared::minwindef::BYTE;
            
            let hdc = GetDC(std::ptr::null_mut());
            let width = GetSystemMetrics(SM_CXSCREEN);
            let height = GetSystemMetrics(SM_CYSCREEN);
            
            let mem_dc = CreateCompatibleDC(hdc);
            let bitmap = CreateCompatibleBitmap(hdc, width, height);
            SelectObject(mem_dc, bitmap as _);
            
            BitBlt(mem_dc, 0, 0, width, height, hdc, 0, 0, SRCCOPY);
            
            ReleaseDC(std::ptr::null_mut(), hdc);
            DeleteDC(mem_dc);
            DeleteObject(bitmap as _);
        }
        
        Ok("screenshot_captured".to_string())
    }

    #[tauri::command]
    pub async fn list_windows(
        _state: State<'_, AppState>,
    ) -> Result<Vec<WindowInfo>, String> {
        let mut windows = Vec::new();
        
        unsafe {
            use winapi::um::winuser::{EnumWindows, GetWindowTextW, GetWindowTextLengthW, GetClassNameW, GetWindowRect, GetWindowThreadProcessId, GetForegroundWindow, IsWindowVisible};
            
            extern "system" fn callback(hwnd: winapi::shared::windef::HWND, lparam: isize) -> i32 {
                let windows: &mut Vec<WindowInfo> = unsafe { &mut *(lparam as *mut Vec<WindowInfo>) };
                
                if IsWindowVisible(hwnd) == 0 {
                    return 1;
                }
                
                let mut title_buf = [0u16; 512];
                let title_len = GetWindowTextW(hwnd, title_buf.as_mut_ptr(), 512);
                let title = if title_len > 0 {
                    String::from_utf16_lossy(&title_buf[..title_len as usize])
                } else {
                    String::new()
                };
                
                if title.is_empty() {
                    return 1;
                }
                
                let mut class_buf = [0u16; 256];
                let class_len = GetClassNameW(hwnd, class_buf.as_mut_ptr(), 256);
                let class_name = if class_len > 0 {
                    String::from_utf16_lossy(&class_buf[..class_len as usize])
                } else {
                    String::new()
                };
                
                let mut rect = std::mem::zeroed::<winapi::shared::minwindef::RECT>();
                GetWindowRect(hwnd, &mut rect);
                
                let mut pid: u32 = 0;
                GetWindowThreadProcessId(hwnd, &mut pid);
                
                let is_active = hwnd == GetForegroundWindow();
                
                windows.push(WindowInfo {
                    handle: hwnd as u64,
                    title,
                    class_name,
                    process_id: pid,
                    is_active,
                    rect: Rect {
                        left: rect.left,
                        top: rect.top,
                        right: rect.right,
                        bottom: rect.bottom,
                    },
                });
                
                1
            }
            
            let mut windows_vec = Vec::new();
            EnumWindows(Some(callback), &mut windows_vec as *mut Vec<WindowInfo> as isize);
            windows = windows_vec;
        }
        
        Ok(windows)
    }

    #[tauri::command]
    pub async fn focus_window(
        handle: u64,
        _state: State<'_, AppState>,
    ) -> Result<(), String> {
        unsafe {
            use winapi::um::winuser::{SetForegroundWindow, ShowWindow, SW_RESTORE};
            let hwnd = handle as winapi::shared::windef::HWND;
            ShowWindow(hwnd, SW_RESTORE);
            SetForegroundWindow(hwnd);
        }
        Ok(())
    }
}
