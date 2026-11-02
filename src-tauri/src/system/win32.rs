// Win32 API bindings for system integration
// This wraps common Windows API calls needed by Eta

pub fn get_foreground_window_title() -> Option<String> {
    unsafe {
        use winapi::um::winuser::{GetForegroundWindow, GetWindowTextW};
        
        let hwnd = GetForegroundWindow();
        if hwnd.is_null() {
            return None;
        }
        
        let mut buf = [0u16; 512];
        let len = GetWindowTextW(hwnd, buf.as_mut_ptr(), 512);
        if len > 0 {
            Some(String::from_utf16_lossy(&buf[..len as usize]))
        } else {
            None
        }
    }
}

pub fn get_clipboard_text() -> Option<String> {
    unsafe {
        use winapi::um::winuser::{OpenClipboard, GetClipboardData, CloseClipboard, EmptyClipboard, SetClipboardData};
        use winapi::um::winnt::CF_TEXT;
        use winapi::um::memoryapi::GlobalLock;
        use winapi::um::handleapi::GlobalUnlock;
        
        if OpenClipboard(None) == 0 {
            return None;
        }
        
        let hdata = GetClipboardData(CF_TEXT);
        if hdata.is_null() {
            CloseClipboard();
            return None;
        }
        
        let ptr = GlobalLock(hdata) as *const u8;
        if ptr.is_null() {
            CloseClipboard();
            return None;
        }
        
        let mut bytes = Vec::new();
        let mut i = 0;
        loop {
            let b = *ptr.add(i);
            if b == 0 { break; }
            bytes.push(b);
            i += 1;
        }
        
        GlobalUnlock(hdata);
        CloseClipboard();
        
        String::from_utf8(bytes).ok()
    }
}

pub fn set_clipboard_text(text: &str) -> bool {
    unsafe {
        use winapi::um::winuser::{OpenClipboard, EmptyClipboard, SetClipboardData, CloseClipboard};
        use winapi::um::winnt::CF_TEXT;
        use winapi::um::memoryapi::{GlobalAlloc, GlobalLock, GlobalUnlock};
        use winapi::um::winbase::GMEM_MOVEABLE;
        
        if OpenClipboard(None) == 0 {
            return false;
        }
        
        EmptyClipboard();
        
        let bytes = text.as_bytes();
        let size = bytes.len() + 1; // null terminator
        
        let hglobal = GlobalAlloc(GMEM_MOVEABLE, size);
        if hglobal.is_null() {
            CloseClipboard();
            return false;
        }
        
        let ptr = GlobalLock(hglobal) as *mut u8;
        if ptr.is_null() {
            CloseClipboard();
            return false;
        }
        
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len());
        *ptr.add(bytes.len()) = 0;
        
        GlobalUnlock(hglobal);
        SetClipboardData(CF_TEXT, hglobal as _);
        CloseClipboard();
        
        true
    }
}

pub fn send_keys(keys: &str) {
    // Simulate keyboard input using SendInput
    unsafe {
        use winapi::um::winuser::{SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP};
        use winapi::um::winuser::MapVirtualKeyA;
        
        for ch in keys.chars() {
            let vk = MapVirtualKeyA(ch as u32, 0) as u16;
            
            let inputs = [
                INPUT {
                    type_: INPUT_KEYBOARD,
                    u: std::mem::zeroed(),
                },
                INPUT {
                    type_: INPUT_KEYBOARD,
                    u: std::mem::zeroed(),
                },
            ];
            
            unsafe {
                let input1 = &mut *(inputs[0].u.keybd_mut());
                input1.wVk = vk;
                input1.wScan = 0;
                input1.dwFlags = 0;
                input1.time = 0;
                input1.dwExtraInfo = 0;
                
                let input2 = &mut *(inputs[1].u.keybd_mut());
                input2.wVk = vk;
                input2.wScan = 0;
                input2.dwFlags = KEYEVENTF_KEYUP;
                input2.time = 0;
                input2.dwExtraInfo = 0;
            }
            
            SendInput(2, inputs.as_ptr(), std::mem::size_of::<INPUT>() as i32);
        }
    }
}
