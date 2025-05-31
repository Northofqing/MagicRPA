use crate::rpa_engine::rpa_automation::win32_component::Win32Component;
use crate::rpa_engine::rpa_core::rect::MagicRect;
use crate::rpa_engine::rpa_core::{error::Result, pinvoke};
use std::cmp::PartialEq;
use windows::Win32::Foundation::HWND;

#[derive(Clone)]
pub struct Win32Control {
    pub component: Win32Component,
}

impl Win32Control {
    pub fn new(handle: HWND) -> Self {
        Self {
            component: Win32Component::new(handle),
        }
    }
}

impl PartialEq for Win32Control {
    fn eq(&self, other: &Self) -> bool {
        self.handle() == other.handle()
    }
}
impl Win32Control {
    pub fn handle(&self) -> HWND {
        self.component.handle()
    }

    pub fn class_name(&self) -> String {
        self.component.class_name()
    }

    pub fn title(&self) -> String {
        self.component.title()
    }

    pub fn is_visible(&self) -> bool {
        self.component.is_visible()
    }

    pub fn is_enabled(&self) -> bool {
        self.component.is_enabled()
    }

    pub fn bounding(&self) -> MagicRect {
        self.component.bounding()
    }

    pub fn process_id(&mut self) -> u32 {
        self.component.process_id()
    }

    // fn clone_box(&self) -> Box<dyn RPAWin32Component> {
    //     todo!()
    // }
}
