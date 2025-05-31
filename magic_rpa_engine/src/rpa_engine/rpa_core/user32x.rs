use windows::{
    core::VARIANT,
    Win32::{
        Foundation::*,
        Graphics::{Dwm::*, Gdi::*},
        System::{Com::*, SystemInformation::*, Threading::*},
        UI::{Accessibility::*, Input::KeyboardAndMouse::*, WindowsAndMessaging::*},
    },
};
use crate::rpa_engine::rpa_core::pinvoke;
fn get_os_version_major() -> u32 {
    let mut version_info: OSVERSIONINFOW = unsafe { std::mem::zeroed() };
    version_info.dwOSVersionInfoSize = std::mem::size_of::<OSVERSIONINFOW>() as u32;

    unsafe {
        GetVersionExW(&mut version_info);
    }
    version_info.dwMajorVersion
}
pub fn is_invisible_win10_background(hWnd: HWND) -> bool {
    if get_os_version_major() < 6 {
        return false;
    }
    let CloakedVal = 0;
    match pinvoke::dwm_get_window_attribute::<RECT>(hWnd, 14) {
        Ok(rect) => {}
        Err(e) => {} //CloakedVal = 0;
    }
    return CloakedVal != 0;
}