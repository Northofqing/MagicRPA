use crate::rpa_engine::rpa_automation::rpa_trace::Trace;
use crate::rpa_engine::rpa_automation::rpa_win32::win32automation::UIFramework;
use crate::rpa_engine::rpa_automation::rpa_win32::win32automation::Win32Automation;
use crate::rpa_engine::rpa_automation::win32_control::Win32Control;
use crate::rpa_engine::rpa_automation::win32_window::Win32Window;
use crate::rpa_engine::rpa_common::selector::Selector;
use crate::rpa_engine::rpa_core::pinvoke;
use crate::rpa_engine::rpa_core::point::MagicPoint;
use crate::rpa_engine::rpa_core::rect::MagicRect;
use std::any::Any;
use std::collections::HashSet;
use std::ffi::c_void;
use std::ptr;
use windows::Win32::Foundation::{HWND, POINT};

use super::element_info::ElementInfo;
use super::matched_spy::MatchedSpy;
use super::rpa_spy::ISpy;
use super::spy_context::SpyContext;
use super::spy_target::SpyTarget;
use super::trace_data::TraceData;
use super::ui_spy_settings::UISpySettings;

pub struct UISpy {
    spies: Vec<Box<dyn ISpy>>,
    win32_spy: Win32Automation,
    focused_hwnd: HWND,
    settings: Option<UISpySettings>,
    matched_spy: Option<MatchedSpy>,
}

impl UISpy {
    pub fn new(spies: Vec<Box<dyn ISpy>>) -> Self {
        let win32_spy = spies
            .last()
            .and_then(|spy| spy.as_any().downcast_ref::<Win32Automation>())
            .expect("Last spy must be Win32Automation")
            .clone();

        Self {
            spies,
            win32_spy,
            focused_hwnd: HWND::default(),
            settings: None,
            matched_spy: None,
        }
    }

    pub fn config(&mut self, settings: UISpySettings) {
        self.settings = Some(settings);
    }

    pub fn trace(&mut self, point: MagicPoint) -> Option<TraceData> {
        let ignore_processes = self
            .settings
            .as_ref()
            .and_then(|s| s.ignore_processes.as_ref());

        let mut wnd = Win32Window::from_point(point, ignore_processes.cloned()).unwrap();

        let context = SpyContext {
            window: wnd.clone(),
            control: Win32Control::new(wnd.handle()),
            point: Some(point),
            target: SpyTarget::Point,
        };

        let spy = self.resolve_spy(&context)?;

        let mut trace_info = spy.trace(&context)?;

        //trace_info.process_id = wnd.component.process_id();
        //trace_info.class_name = wnd.component.class_name();

        Some(trace_info)
    }

    pub fn window_from_focus(&self) -> HWND {
        unsafe {
            let mut focused_handle = pinvoke::get_focus();

            if focused_handle == HWND::default() {
                let foreground_hwnd = pinvoke::get_foreground_window();
                if foreground_hwnd != HWND::default() {
                    let current_tid = pinvoke::get_current_threadId();
                    let tid = pinvoke::get_window_thread_processId(foreground_hwnd);

                    if pinvoke::attach_thread_input(current_tid, tid, true) {
                        focused_handle = pinvoke::get_focus();
                        pinvoke::attach_thread_input(current_tid, tid, false);
                    }
                }
            }
            focused_handle
        }
    }

    fn resolve_spy(&mut self, context: &SpyContext) -> Option<&dyn ISpy> {
        // Check if we can reuse the matched spy
        if let Some(matched) = &self.matched_spy {
            if matched.target == context.target
                && context.control.handle() != HWND::default()
                && context.control == Win32Control::new(matched.hwnd)
            {
                return Some(matched.spy.as_ref());
            }
        }

        // Find a matching spy
        for spy in &self.spies {
            if spy.is_match(context) {
                if !spy.as_any().is::<Win32Automation>() {
                    if let Some(matched) = &self.matched_spy {}
                    // self.matched_spy = Some(MatchedSpy {
                    //     hwnd: context.control.handle(),
                    //     spy: spy.to_owned(),
                    //     target: context.target,
                    // });
                }
                return Some(spy.as_ref());
            }
        }

        None
    }

    fn should_ignore_process(&self, pid: u32) -> bool {
        self.settings
            .as_ref()
            .and_then(|s| s.ignore_processes.as_ref())
            .map(|processes| processes.contains(&pid))
            .unwrap_or(false)
    }

    // Other methods would be implemented similarly...
}

// Required trait implementations
impl Clone for Box<dyn ISpy> {
    fn clone(&self) -> Self {
        self.clone()
    }
}

trait ISpyClone {
    fn clone_box(&self) -> Box<dyn ISpy>;
}

impl<T> ISpyClone for T
where
    T: 'static + ISpy + Clone,
{
    fn clone_box(&self) -> Box<dyn ISpy> {
        Box::new(self.clone())
    }
}
