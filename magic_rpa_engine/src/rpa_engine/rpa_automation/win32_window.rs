use crate::rpa_engine::rpa_core::{
    error::Result,
    pinvoke,
    point::MagicPoint,
    rect::MagicRect,
    rpa_enum::ExtendedWindowStyles::{self, *},
    user32plus, user32x,
};
use windows::Win32::Foundation::HWND;
use crate::rpa_engine::rpa_automation::win32_component::Win32Component;

use autoit_wrapper::{autoit_error::AutoItError, AutoIt};

pub struct Win32Window {
    pub component: Win32Component,
    hwnd: HWND,
    is_java_popup: bool,
}
impl Clone for Win32Window {
    fn clone(&self) -> Self {
        Win32Window {
            component: Win32Component::new(self.hwnd),
            hwnd: self.hwnd.clone(),
            is_java_popup: self.is_java_popup.clone(),
        }
    }
}
impl Win32Window {
    pub fn new(handle: HWND) -> Self {
        Self {
            component: Win32Component::new(handle),
            hwnd: handle,
            is_java_popup: false,
        }
    }
    pub fn handle(&self) -> HWND {
        self.hwnd
    }
    pub fn text(&self) -> String {
        AutoIt::win_get_text_by_handle(self.hwnd).unwrap()
    }

    pub fn is_iconic(&self) -> bool {
        pinvoke::is_iconic(self.hwnd)
    }
    pub fn is_layered(&self) -> bool {
        match self.get_extended_style() {
            Ok(style) => style.to_u32() & Style::Layered.to_u32() > Style::Left.to_u32(),
            Err(e) => {
                log::warn!("get window: {:#?} info failure: {}", self.hwnd, e);
                false
            }
        }
    }

    // pub fn is_transparent(&self) -> bool {
    //     match self.get_extended_style() {
    //         Ok(style) => style & WINDOW_EX_STYLE(WS_EX_TRANSPAREMagic.0) > WINDOW_EX_STYLE(WS_EX_LEFT.0),
    //         Err(e) => {
    //             log::warn!("get window: {:x} info failure: {}", self.hwnd.0, e);
    //             false
    //         }
    //     }
    // }

    // pub fn is_no_activate(&self) -> bool {
    //     match self.get_extended_style() {
    //         Ok(style) => style & WINDOW_EX_STYLE(WS_EX_NOACTIVATE.0) > WINDOW_EX_STYLE(WS_EX_LEFT.0),
    //         Err(e) => {
    //             log::warn!("get window: {:x} info failure: {}", self.hwnd.0, e);
    //             false
    //         }
    //     }
    // }

    // pub fn is_tool_window(&self) -> bool {
    //     match self.get_extended_style() {
    //         Ok(style) => style & WINDOW_EX_STYLE(WS_EX_TOOLWINDOW.0) > WINDOW_EX_STYLE(WS_EX_LEFT.0),
    //         Err(e) => {
    //             log::warn!("get window: {:x} info failure: {}", self.hwnd.0, e);
    //             false
    //         }
    //     }
    // }

    // pub fn is_topmost(&self) -> bool {
    //     match self.get_extended_style() {
    //         Ok(style) => style & WINDOW_EX_STYLE(WS_EX_TOPMOST.0) > WINDOW_EX_STYLE(WS_EX_LEFT.0),
    //         Err(e) => {
    //             log::warn!("get window: {:x} info failure: {}", self.hwnd.0, e);
    //             false
    //         }
    //     }
    // }

    // pub fn style(&self) -> WINDOW_STYLE {
    //     unsafe {
    //         WINDOW_STYLE(GetWindowLongW(self.hwnd, GWL_STYLE) as u32)
    //     }
    // }

    // pub fn extended_style(&self) -> WINDOW_EX_STYLE {
    //     unsafe {
    //         WINDOW_EX_STYLE(GetWindowLongW(self.hwnd, GWL_EXSTYLE) as u32)
    //     }
    // }

    // pub fn activate(&self) -> Result<(), Box<dyn std::error::Error>> {
    //     const TIMEOUT: Duration = Duration::from_millis(2000);

    //     // Skip activation for special windows
    //     if self.component.class_name() == "#32768"
    //         || self.component.class_name() == "ComboLBox"
    //         || self.is_java_popup {
    //         return Ok(());
    //     }

    //     // Special handling for desktop and explorer windows
    //     if self.component.class_name() == "#32769"
    //         || (self.component.process_name() == "explorer"
    //             && (self.component.class_name() == "WorkerW"
    //                 || self.component.class_name() == "Progman")) {
    //         // Send Windows+M equivalent
    //         // Note: This would need a proper implementation
    //         thread::sleep(TIMEOUT);
    //         return Ok(());
    //     }

    //     // Skip Chrome widget windows
    //     if self.component.class_name() == "Chrome_WidgetWin_2" {
    //         return Ok(());
    //     }

    //     let win_handle = if self.is_java_window() {
    //         self.get_last_java_popup_window()
    //     } else {
    //         self.hwnd
    //     };

    //     self.retry_activate(win_handle, TIMEOUT)?;
    //     Ok(())
    // }

    pub fn is_active(&self) -> bool {
        pinvoke::get_active_window() == self.hwnd
    }

    pub fn is_desktop(&self) -> bool {
        self.component.class_name() == "#32769"
    }

    pub fn is_chrome_widget_win(&self) -> bool {
        self.component.class_name().contains("Chrome_WidgetWin")
    }

    pub fn is_click_through(&self) -> bool {
        if !self.is_layered() {
            return false;
        }

        // if self.is_transparent() {
        //     return true;
        // }

        // if self.is_no_activate() {
        //     return self.component.find_descendant(|_| true).is_none();
        // }

        false
    }

    // pub fn set_state(&mut self, flag: &str) -> Result<(), Box<dyn std::error::Error>> {
    //     match flag.parse::<i32>() {
    //         Ok(state) => {
    //             // Implementation would need proper window state setting
    //             Ok(())
    //         },
    //         Err(_) => {
    //             log::error!("set state parameter wrong, value: {}", flag);
    //             Err("Invalid window state parameter".into())
    //         }
    //     }
    // }

    // pub fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
    //     unsafe {
    //         if windows::Win32::UI::WindowsAndMessaging::PostMessageW(
    //             self.hwnd,
    //             windows::Win32::UI::WindowsAndMessaging::WM_CLOSE,
    //             None,
    //             None,
    //         ).is_ok() {
    //             Ok(())
    //         } else {
    //             Err("Failed to close window".into())
    //         }
    //     }
    // }

    // pub fn exists(&self) -> bool {
    //     unsafe { windows::Win32::UI::WindowsAndMessaging::IsWindow(self.hwnd).as_bool() }
    // }

    // pub fn wait_exists(&self, timeout: Duration) -> Result<bool, Box<dyn std::error::Error>> {
    //     let start = std::time::Instant::now();
    //     while start.elapsed() < timeout {
    //         if self.exists() {
    //             return Ok(true);
    //         }
    //         thread::sleep(Duration::from_millis(100));
    //     }
    //     Ok(false)
    // }

    // pub fn wait_close(&self, timeout: Duration) -> Result<bool, Box<dyn std::error::Error>> {
    //     let start = std::time::Instant::now();
    //     while start.elapsed() < timeout {
    //         if !self.exists() {
    //             return Ok(true);
    //         }
    //         thread::sleep(Duration::from_millis(100));
    //     }
    //     Ok(false)
    // }

    // pub fn move_window(&self, x: i32, y: i32) -> Result<(), Box<dyn std::error::Error>> {
    //     unsafe {
    //         if windows::Win32::UI::WindowsAndMessaging::SetWindowPos(
    //             self.hwnd,
    //             None,
    //             x,
    //             y,
    //             0,
    //             0,
    //             windows::Win32::UI::WindowsAndMessaging::SWP_NOSIZE |
    //             windows::Win32::UI::WindowsAndMessaging::SWP_NOZORDER
    //         ).is_ok() {
    //             Ok(())
    //         } else {
    //             Err("Failed to move window".into())
    //         }
    //     }
    // }

    // pub fn set_size(&self, width: i32, height: i32) -> Result<(), Box<dyn std::error::Error>> {
    //     let rect = self.component.bounding();
    //     unsafe {
    //         if windows::Win32::UI::WindowsAndMessaging::SetWindowPos(
    //             self.hwnd,
    //             None,
    //             rect.left,
    //             rect.top,
    //             width,
    //             height,
    //             windows::Win32::UI::WindowsAndMessaging::SWP_NOMOVE |
    //             windows::Win32::UI::WindowsAndMessaging::SWP_NOZORDER
    //         ).is_ok() {
    //             Ok(())
    //         } else {
    //             Err("Failed to resize window".into())
    //         }
    //     }
    // }

    // pub fn send_keys(&self, text: &str, contains_hotkey: bool, send_key_delay: u32)
    //     -> Result<(), Box<dyn std::error::Error>> {
    //     // Note: This would need a proper implementation for sending keystrokes
    //     // For now just log the attempt
    //     log::debug!(
    //         "Would send keys: '{}' (hotkey: {}, delay: {}ms)",
    //         text,
    //         contains_hotkey,
    //         send_key_delay
    //     );
    //     Ok(())
    // }

    // // Helper methods
    fn get_extended_style(&self) -> Result<Style> {
        Ok(Style::from_u32(
            pinvoke::get_window_long(self.hwnd, -20) as u32
        ))
    }

    // fn retry_activate(&self, window: HWND, timeout: Duration) -> Result<(), Box<dyn std::error::Error>> {
    //     let start = std::time::Instant::now();
    //     while start.elapsed() < timeout {
    //         if self.is_active() {
    //             return Ok(());
    //         }

    //         unsafe {
    //             windows::Win32::UI::WindowsAndMessaging::SetForegroundWindow(window);
    //         }

    //         thread::sleep(Duration::from_millis(200));
    //     }
    //     Err("Failed to activate window".into())
    // }

    // // Static methods
    // pub fn get_active_window() -> Result<Win32Window, Box<dyn std::error::Error>> {
    //     unsafe {
    //         let hwnd = windows::Win32::UI::WindowsAndMessaging::GetActiveWindow();
    //         if hwnd.is_invalid() {
    //             Err("No active window found".into())
    //         } else {
    //             Ok(Win32Window::new(hwnd))
    //         }
    //     }
    // }

    pub fn get_desktop_window() -> Win32Window {
        let desktop = pinvoke::get_desktop_window();
        Win32Window::new(desktop.unwrap())
    }
    fn match_desktop_child_predicate<F>(
        hWnd: HWND,
        desktopHWnd: HWND,
        predicate: F,
        includeInvisible: bool,
    ) -> Win32Window
    where
        F: Fn(&mut Win32Window) -> bool,
    {
        if pinvoke::is_window(hWnd) {
            if (includeInvisible || pinvoke::is_window_visible(hWnd))
                && pinvoke::get_ancestor(hWnd, 0).unwrap() == desktopHWnd
            {
                let mut win32Window = Win32Window::new(hWnd);
                user32x::is_invisible_win10_background(hWnd);

                if predicate(&mut win32Window) {
                    return win32Window.clone();
                }
            }
        }
        Win32Window::new(HWND::default())
    }

    pub fn find_window_all<F>(
        &self,
        predicate: &F,
        include_invisible: bool,
    ) -> Option<Vec<Win32Window>>
    where
        F: Fn(&mut Win32Window) -> bool,
    {
        let mut windows = Vec::new();
        let hwnds = user32plus::query_all_child_windows(HWND::default(), |hwnd| {
            let mut windows = Vec::new();
            let wnd = Win32Window::new(hwnd);
            let desktop = pinvoke::get_desktop_window();
            let is_desktop = wnd.is_desktop();
            if is_desktop {
                let wnd = Win32Window::match_desktop_child_predicate(
                    hwnd,
                    desktop.unwrap(),
                    predicate,
                    include_invisible,
                );
                windows.push(wnd);
            } else {
                // let wnd = Win32Window::match_desktop_child_predicate(
                //     HWND::default(),
                //     desktop.unwrap(),
                //     predicate,
                //     include_invisible,
                // );
                //windows.push(wnd);
            }
            return false;
        });
        Some(windows)
    }

    pub fn from_point(
        point: MagicPoint,
        ignore_processes: Option<Vec<u32>>,
    ) -> Option<Win32Window> {
        let desktop_hwnd = pinvoke::get_desktop_window().unwrap();
        let desktop = Win32Window::new(desktop_hwnd);
        desktop.find_window_all(
            &|wnd: &mut Win32Window| {
                let is_ignore_processes = match &ignore_processes {
                    Some(processes) => !processes.contains(&wnd.component.process_id()),
                    None => true,
                };
                if wnd.is_iconic()
                    && wnd.is_click_through()
                    && is_ignore_processes
                    && wnd.component.bounding().contain(point)
                {
                    return true;
                }
                false
            },
            false,
        );

        Some(Win32Window::new(desktop_hwnd))
    }
}
