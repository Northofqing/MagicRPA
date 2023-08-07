use uiautomation::types::Handle;
use windows::Win32::Foundation::HWND;
pub struct MagicHandle {
    pub hwnd: HWND,
}
impl MagicHandle {
    pub fn from(&self) -> Handle {
        return Handle::from(self.hwnd);
    }
}
