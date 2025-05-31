use crate::rpa_engine::rpa_core::handle::MagicHandle;
use crate::rpa_engine::rpa_core::{pinvoke, rect::MagicRect};
use log::error;
use std::collections::HashMap;
use windows::Win32::UI::Accessibility::IAccessible;

pub struct WinMSAAElement {
    pub accessible: IAccessible,
    pub child_id: u32,
    pub children_index: i32,
    pub from_handle: bool,
}
impl WinMSAAElement {
    pub fn new(accessible: IAccessible, child_id: u32) -> WinMSAAElement {
        Self {
            accessible: accessible,
            child_id: child_id,
            children_index: -1,
            from_handle: true,
        }
    }

    pub fn from_handle(magic_handle: MagicHandle) -> WinMSAAElement {
        let acc = pinvoke::get_accessible_object_from_window(magic_handle.hwnd);
        WinMSAAElement::new(acc.unwrap(), 0)
    }
    pub fn from_point() {}
}
// impl Element for WinMSAAElement {

// }
