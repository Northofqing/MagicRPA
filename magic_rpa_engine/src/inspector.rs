use crate::rpa_engine::rpa_automation::rpa_trace::RPATrace;
use crate::rpa_engine::rpa_common::selector::{self, SelectorProps};
use crate::rpa_engine::rpa_core::point::MagicPoint;
use std::time::Instant;
use uiautomation::UIAutomation;

use log::{info, warn};
pub struct Inspector {}
impl Inspector {
    pub fn start() {
       Self::inspect_element_point();
    }
    pub fn stop() {}
    fn inspect_element_point() {
        loop {
            // 开始计时
            let start = Instant::now();
            let point = MagicPoint::get_cursor_pos();
            let automation = UIAutomation::new();
            let _automation =
                crate::rpa_engine::rpa_automation::rpa_win32::win32automation::Win32Automation {
                    automation: automation.unwrap(),
                };
            let _element = _automation.trace_element_point(point);
            let _name = _element.get_name();
            let _classname = _element.get_classname();
            let _rect = _element.get_bounding_rectangle();
            let _left = _rect.x;
            let _top = _rect.y;
            let _width = _rect.width;
            let _height = _rect.height;
            let _type = _element.get_control_type();
            // 结束计时并计算经过的时间
            let duration = start.elapsed();
            info!("name:{_name}");
            info!("classname:{_classname}");
            info!("left:{_left} top:{_top} width:{_width} height:{_height}");
            info!("Elapsed time: {:?}", duration);
        }
    }
}
