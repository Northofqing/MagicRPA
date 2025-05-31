use crate::rpa_engine::{rpa_automation::{win32_control::Win32Control, win32_window::Win32Window}, rpa_core::point::MagicPoint};

use spy_target::SpyTarget;

use super::spy_target;

pub struct SpyContext {
    pub(crate) window: Win32Window,
    pub(crate) control: Win32Control,
    pub(crate) point: Option<MagicPoint>,
    pub(crate) target: SpyTarget,
}