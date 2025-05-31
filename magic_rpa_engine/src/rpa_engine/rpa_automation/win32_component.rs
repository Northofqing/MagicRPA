use crate::rpa_engine::rpa_automation::win32_control::Win32Control;
use crate::rpa_engine::rpa_automation::win32_window::Win32Window;
use crate::rpa_engine::rpa_core::{error::Result, pinvoke, point::MagicPoint, rect::MagicRect};
use std::ffi::c_void;
use std::ptr;
use std::ptr::null;
use std::sync::Once;
use windows::Win32::Foundation::HWND;

use autoit_wrapper::{autoit_error::AutoItError, AutoIt};

// impl Win32Component {
//     fn handle(&self) -> HWND {
//         self.h_wnd
//     }
//     fn class_name(&self) -> String {
//         pinvoke::get_class_name(self.h_wnd).unwrap()
//     }
//     fn is_visible(&self) -> bool {
//         pinvoke::is_window_visible(self.h_wnd)
//     }

//     fn is_enabled(&self) -> bool {
//         pinvoke::is_window_enable(self.h_wnd)
//     }

//     fn title(&self) -> String {
//         AutoIt::win_get_title_by_handle(self.h_wnd).unwrap()
//     }

//     fn bounding(&self) -> MagicRect {
//         let result = pinvoke::get_window_rect(self.h_wnd).unwrap();
//         MagicRect::new(result.0, result.1, result.2 - result.0, result.3 - result.1)
//     }
//     fn process_id(&mut self) -> u32 {
//         if self.process_id.is_none() {
//             self.process_id = Some(pinvoke::get_window_thread_processId(self.h_wnd));
//         }
//         self.process_id.unwrap()
//     }

//     // fn clone_box(&self) -> Box<dyn RPAWin32Component> {
//     //     todo!()
//     // }
// }

pub struct Win32Component {
    h_wnd: HWND,
    root: HWND,
    root_owner: HWND,
    owner: HWND,
    parent: HWND,
    process_id: Option<u32>,
    process_name: Option<String>,
}
impl Clone for Win32Component {
    fn clone(&self) -> Self {
        Win32Component {
            h_wnd: self.h_wnd.clone(),
            root: self.root.clone(),
            root_owner: self.root_owner.clone(),
            owner: self.owner.clone(),
            parent: self.parent.clone(),
            process_id: self.process_id.clone(),
            process_name: self.process_name.clone(),
        }
    }
}
impl Win32Component {
    pub fn new(handle: HWND) -> Self {
        Self {
            h_wnd: handle,
            root: handle,
            root_owner: handle,
            owner: handle,
            parent: handle,
            process_id: None,
            process_name: None,
        }
    }
    pub fn screen_to_client(&self, point: MagicPoint) -> MagicPoint {
        let _p = pinvoke::screen_to_client(self.h_wnd, point.x, point.y).unwrap();
        MagicPoint::new(_p.0, _p.1)
    }
    pub fn client_to_screen(&self, point: MagicPoint) -> MagicPoint {
        let _p = pinvoke::client_to_screen(self.h_wnd, point.x, point.y).unwrap();
        MagicPoint::new(_p.0, _p.1)
    }
    // pub fn find_descendant<F>(&self, predicate: F) -> Option<Win32Control>
    // where
    //     F: Fn(&Win32Control) -> bool,
    // {
    //     unsafe {
    //         // Implementation would need a custom EnumChildWindows wrapper
    //         // This is a simplified version
    //         let mut result = None;
    //         windows::Win32::UI::WindowsAndMessaging::EnumChildWindows(
    //             self.h_wnd,
    //             Some(enum_child_proc),
    //             windows::Win32::Foundation::LPARAM(&mut result as *mut _ as isize),
    //         );
    //         result
    //     }
    // }
    pub fn children<F>(&self, predicate: F) -> Vec<Win32Control>
    where
        F: Fn(&Win32Control) -> bool,
    {
        let mut children = vec![];
        let mut child = pinvoke::get_child_window(self.h_wnd);
        while child != HWND::default() {
            let child_window = Win32Control::new(child);
            if predicate(&child_window) {
                children.push(child_window);
            }
            child = pinvoke::get_next_window(child);
        }
        children
    }
    // fn get_root_owner(&mut self) -> Win32Window {
    //     if self.owner.is_none() {
    //         pinvoke::get_ancestor(self.h_wnd, 3);
    //         unsafe {
    //             let hwnd = pinvoke::get_ancestor(self.h_wnd, 3).unwrap();
    //             return Win32Window::new(hwnd);
    //         }
    //     }
    //     let mut current = self.owner.as_ref().unwrap();
    //     while let Some(owner) = current.owner.as_ref() {
    //         current = owner;
    //     }
    //     current.clone()
    // }

    pub fn handle(&self) -> HWND {
        self.h_wnd
    }

    // pub fn root(&mut self) -> &Win32Window {
    //     if self.root != HWND::default() {
    //         unsafe {
    //             let root_hwnd = pinvoke::get_ancestor(self.root, 2).unwrap();
    //             &Win32Window::new(root_hwnd)
    //         }
    //     }
    // }

    // fn root_owner(&mut self) -> &Win32Window {
    //     if self.root_owner.is_none() {
    //         self.root_owner = Some(self.get_root_owner());
    //     }
    //     self.root_owner.as_ref().unwrap()
    // }

    // pub fn owner(&mut self) -> Option<&Win32Window> {
    //     if self.owner.is_none() {
    //         unsafe {
    //             let desktop = pinvoke::get_desktop_window().unwrap();
    //             let owner_hwnd = pinvoke::get_window(self.h_wnd, 4).unwrap();

    //             if owner_hwnd.is_invalid() || owner_hwnd == desktop {
    //                 return None;
    //             }

    //             self.owner = Some(Win32Window::new(owner_hwnd));
    //         }
    //     }
    //     self.owner.as_ref()
    // }

    // fn parent(&mut self) -> Option<&Box<dyn Win32ComponentTrait>> {
    //     if self.parent.is_none() {
    //         unsafe {
    //             let parent_hwnd = pinvoke::get_ancestor(self.h_wnd, 1).unwrap();
    //             if parent_hwnd.is_invalid() {
    //                 return None;
    //             }

    //             let ancestor = GetAncestor(parent_hwnd, GA_PAREMagic);
    //             if ancestor.is_invalid() || ancestor == pinvoke::get_desktop_window().unwrap() {
    //                 self.parent = Some(Box::new(Win32Window::new(parent_hwnd)));
    //             } else {
    //                 self.parent = Some(Box::new(Win32Control::new(parent_hwnd)));
    //             }
    //         }
    //     }
    //     self.parent.as_ref()
    // }

    pub fn bounding(&self) -> MagicRect {
        let result = pinvoke::get_window_rect(self.h_wnd).unwrap();
        MagicRect::new(result.0, result.1, result.2 - result.0, result.3 - result.1)
    }

    pub fn client_bounding(&self) -> MagicRect {
        let result = pinvoke::get_client_rect(self.h_wnd).unwrap();
        MagicRect::new(result.0, result.1, result.2 - result.0, result.3 - result.1)
    }

    // fn is_top_level_window(&mut self) -> bool {
    //     let owner = self.owner();
    //     owner.is_none()
    //         || !Win32Component::is_win32_window(owner.unwrap().handle())
    //         || !owner.unwrap().is_visible()
    //         || owner.unwrap().client_bounding().right == 0
    // }

    pub fn is_visible(&self) -> bool {
        pinvoke::is_window_visible(self.h_wnd)
    }

    pub fn is_enabled(&self) -> bool {
        pinvoke::is_window_enable(self.h_wnd)
    }

    pub fn class_name(&self) -> String {
        pinvoke::get_class_name(self.h_wnd).unwrap()
    }

    pub fn title(&self) -> String {
        "".to_string()
        // unsafe {
        //     let mut buffer = [0u16; 65535];
        //     let len = windows::Win32::UI::WindowsAndMessaging::GetWindowText(
        //         self.h_wnd,
        //         &mut buffer,
        //     );
        //     String::from_utf16_lossy(&buffer[..len as usize])
        // }
    }

    pub fn process_id(&mut self) -> u32 {
        if self.process_id.is_none() {
            self.process_id = Some(pinvoke::get_window_thread_processId(self.h_wnd));
        }
        self.process_id.unwrap()
    }

    pub fn process_name(&mut self) -> &str {
        if self.process_name.is_none() {
            let pid = self.process_id();
            // Note: In real implementation, you'd want to use windows-rs Process APIs
            // This is simplified
            self.process_name = Some(format!("Process_{}", pid));
        }
        self.process_name.as_ref().unwrap()
    }
}
