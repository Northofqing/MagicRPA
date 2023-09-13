use crate::rpa_engine::rpa_core::error::Result;
use windows::Win32::Foundation::HWND;
pub struct Win32Compoent {
    pub hwnd: HWND,
}
impl Win32Compoent {
    pub fn new(_hwnd: HWND) -> Win32Compoent {
        Self { hwnd: _hwnd }
    }
    pub fn get_handle(&self) -> Result<HWND> {
        return Ok(self.hwnd);
    }
     
}
