use crate::rpa_engine::rpa_automation::rpa_trace::Trace;
use crate::rpa_engine::rpa_automation::rpa_win32::win32automation::Win32Automation;
//use crate::rpa_engine::rpa_common::selector::{self, SelectorProps};
use crate::rpa_engine::rpa_core::point::MagicPoint;
use std::time::Instant;

extern crate log;

pub struct Inspector {}

impl Inspector {
    pub fn start() {
        Self::inspect_element_point();
    }
    pub fn stop() {}

    pub fn inspect_element_point_single(magic_point: MagicPoint) {
        let mut _automation = Win32Automation::new();
        let _ = _automation.is_match();
        let _element = _automation.trace_element_point(magic_point).unwrap();
        let _ = _element.get_name();
        let _ = _element.get_classname();
        let _ = _element.get_bounding_rectangle();
        let _ = _element.get_control_type();
        let _ = _element.get_index();

        let _ = _element.is_enabled();
        let _ = _element.is_focus();
        let _ = _element.is_edit();

        let _ = _element.get_checked();
        let _ = _element.set_check();
        let _ = _element.get_text();
        let _ = _element.set_text();

        let _ = _element.get_select();
        let _ = _element.set_select();
        let _ = _element.event_click();
        let _ = _element.get_attributes();
        let _ = _element.get_attribute("");
        let _ = _element.set_attribute("", "");

        let _ = _element.get_parent();
        let _ = _element.get_children();
        let _ = _element.get_descendants();

        let _ = _automation.build_selector(_element);
    }
    fn inspect_element_point() {
        loop {
            // 开始计时
            let start = Instant::now();
            let point = MagicPoint::get_cursor_pos();
            Self::inspect_element_point_single(point);
            // 结束计时并计算经过的时间
            let duration = start.elapsed();

            log::info!("Elapsed time: {:?}", duration);
            log::info!("_____分割线______");
        }
    }
}
