use uiautomation::types::Handle;
use windows::Win32::Foundation::HWND;
pub struct MagicHandle {
    pub hwnd: HWND,
}
impl MagicHandle {
    pub fn new(hwnd: HWND) -> MagicHandle {
        Self { hwnd: hwnd }
    }
    pub fn default(&self) -> Handle {
        return Handle::default();
    }
}
