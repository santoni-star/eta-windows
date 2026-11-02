use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceConfig {
    pub enabled: bool,
    pub stt_provider: String,  // "whisper", "azure", "google"
    pub tts_provider: String,  // "edge", "azure", "google", "espeak"
    pub language: String,
    pub voice_id: Option<String>,
}

impl Default for VoiceConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            stt_provider: "whisper".to_string(),
            tts_provider: "edge".to_string(),
            language: "uk-UA".to_string(),
            voice_id: None,
        }
    }
}

pub mod commands {
    use tauri::State;
    use crate::AppState;

    #[tauri::command]
    pub async fn start_listening(
        _state: State<'_, AppState>,
    ) -> Result<(), String> {
        // Start speech-to-text listening
        Ok(())
    }

    #[tauri::command]
    pub async fn stop_listening(
        _state: State<'_, AppState>,
    ) -> Result<(), String> {
        Ok(())
    }

    #[tauri::command]
    pub async fn speak(
        text: String,
        _state: State<'_, AppState>,
    ) -> Result<(), String> {
        // Text-to-speech using Edge TTS or system SAPI
        unsafe {
            use winapi::um::ole2::{CoInitialize, CoUninitialize};
            use winapi::um::sapi::{ISpVoice, CLSID_SpVoice};
            use winapi::um::combaseapi::CoCreateInstance;
            use winapi::um::unknwnbase::IUnknown;
            use winapi::shared::winerror::SUCCEEDED;
            use winapi::um::objbase::CLSCTX_ALL;
            
            CoInitialize(std::ptr::null_mut());
            
            let mut voice: *mut ISpVoice = std::ptr::null_mut();
            let hr = CoCreateInstance(
                &CLSID_SpVoice,
                std::ptr::null_mut(),
                CLSCTX_ALL,
                &ISpVoice::IID,
                &mut voice as *mut *mut ISpVoice as *mut *mut std::ffi::c_void,
            );
            
            if SUCCEEDED(hr) && !voice.is_null() {
                let wide_text: Vec<u16> = text.encode_utf16().collect();
                (*voice).Speak(wide_text.as_ptr(), 0, std::ptr::null_mut());
                (*voice).Release();
            }
            
            CoUninitialize();
        }
        
        Ok(())
    }
}
