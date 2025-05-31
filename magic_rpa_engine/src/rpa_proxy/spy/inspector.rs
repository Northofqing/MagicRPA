use crate::rpa_engine::rpa_automation::rpa_trace::Trace;
use crate::rpa_engine::rpa_automation::rpa_win32::win32automation::Win32Automation;
use crate::rpa_engine::rpa_core::point::MagicPoint;
use crate::rpa_engine::rpa_core::rect::MagicRect;
use std::time::Instant;

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use windows::Win32;
use windows::Win32::System::Com::CoUninitialize;
use windows::Win32::System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED};

use super::rpa_spy;
use super::trace_data::TraceData;
use super::ui_spy::UISpy;
use rpa_spy::ISpy;

// extern crate log;

pub struct InspectorInner {
    _state: bool,
}
impl InspectorInner {
    pub fn new() -> InspectorInner {
        Self { _state: true }
    }
    pub fn inspect_element_point_spy(&self, point: MagicPoint) -> Option<TraceData> {
        let win = Win32Automation::new();
        let mut spies = Vec::new();
        spies.push(Box::new(win) as Box<dyn ISpy>);
        let mut spy = UISpy::new(spies);
        spy.trace(point)
    }

    pub fn inspect_element_point_single(&self, Magic_point: MagicPoint) {
        // let mut _automation = Win32Automation::new();
        // let _ = _automation.is_match();
        // let _element = _automation.trace_element_point(Magic_point).unwrap();

        // let selector = _automation.build_selector(_element);
        // log::info!(" {:?}",selector);
    }

    fn inspect_element_point(&self) {
        loop {
            if !&self._state {
                break;
            }

            // 开始计时
            let start = Instant::now();
            let point = MagicPoint::get_cursor_pos();
            //Self::inspect_element_point_single(point);
            let rect = &self.inspect_element_point_spy(point);
            //log::info!("element bounding: {:?}", rect);
            // 结束计时并计算经过的时间
            let duration = start.elapsed();

            log::info!("Elapsed time: {:?}", duration);
            log::info!("_____分割线______");
            thread::sleep(Duration::from_millis(200));
        }
    }
}
pub struct Inspector {
    inner: Arc<Mutex<InspectorInner>>,
}
impl Inspector {
    // pub fn new() -> Inspector {
    //     Self {
    //         inner: Arc::new(Mutex::new(InspectorInner { _state: false })),
    //     }
    // }
    // pub fn start(&self) {
    //     self.inner.lock().unwrap()._state = true;
    //     let _ = &self.inspect_element_point_thread();
    // }
    // pub fn stop(&self) {
    //     self.inner.lock().unwrap()._state = false;
    // }
    // fn inspect_element_point_spy(Magic_point: MagicPoint) -> MagicRect {
    //     let mut _automation = Win32Automation::new();
    //     let _ = _automation.is_match();
    //     let _element = _automation.trace_element_point(Magic_point).unwrap();
    //     _element.get_bounding_rectangle().unwrap()
    // }

    // pub fn inspect_element_point_single(Magic_point: MagicPoint) {
    //     let mut _automation = Win32Automation::new();
    //     let _ = _automation.is_match();
    //     let _element = _automation.trace_element_point(Magic_point).unwrap();
    //     let _ = _element.get_name();
    //     let _ = _element.get_classname();
    //     let _ = _element.get_bounding_rectangle();
    //     let _ = _element.get_control_type();
    //     let _ = _element.get_index();

    //     let _ = _element.is_enabled();
    //     let _ = _element.is_focus();
    //     let _ = _element.is_edit();

    //     let _ = _element.get_checked();
    //     let _ = _element.set_check();
    //     let _ = _element.get_text();
    //     let _ = _element.set_text();

    //     let _ = _element.get_select();
    //     let _ = _element.set_select();
    //     let _ = _element.event_click();
    //     let _ = _element.get_attributes();
    //     let _ = _element.get_attribute("");
    //     let _ = _element.set_attribute("", "");

    //     let _ = _element.get_parent();
    //     let _ = _element.get_children();
    //     let _ = _element.get_descendants();

    //     let _ = _automation.build_selector(_element);
    // }

    // fn inspect_element_point_thread(&self) {
    //     let shared_self = Arc::new(self);
    //     let self_copy = shared_self.inner.clone();

    //     let handle = thread::spawn(move || {
    //         let local_self = &self_copy;

    //         unsafe {
    //             let result = CoInitializeEx(None, COINIT_APARTMEMagicTHREADED);

    //             if result.is_ok() {
    //                 println!("成功初始化STA线程");
    //                 local_self.lock().unwrap().inspect_element_point();

    //                 // 完成后清理COM
    //                 CoUninitialize();
    //             } else {
    //                 println!("初始化STA线程失败: {:?}", result);
    //             }
    //         }
    //     });

    //     // 等待线程完成
    //     //handle.join().unwrap();
    // }
}
