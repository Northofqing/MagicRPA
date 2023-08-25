use crate::rpa_engine::rpa_automation::rpa_enum::targeFramework;
use crate::rpa_engine::rpa_automation::rpa_trace::RPATrace;
use crate::rpa_engine::rpa_common::selector::{self, SelectorProps};
use crate::rpa_engine::rpa_core::point::MagicPoint;
use std::time::Instant;
use uiautomation::UIAutomation;

extern crate log;

pub struct Inspector {}

impl Inspector {
    pub fn start() {
        Self::inspect_element_point();
    }
    pub fn stop() {}
    fn inspect_element_point() {
        let mut catch_name = String::new();
        loop {
            // 开始计时
            let start = Instant::now();
            let point = MagicPoint::get_cursor_pos();
            let automation = UIAutomation::new();
            let mut _automation =
                crate::rpa_engine::rpa_automation::rpa_win32::win32automation::Win32Automation {
                    automation: automation.unwrap(),
                    target_framework: targeFramework::uia,
                };
            _automation.is_match();
            let _element = _automation.trace_element_point(point);
            let _rect = _element.get_bounding_rectangle();
            let _left = _rect.x;
            let _top = _rect.y;
            let _width = _rect.width;
            let _height = _rect.height;
            let _type = _element.get_control_type();

            let _parent = _element.get_parent();
            // 结束计时并计算经过的时间
            let duration = start.elapsed();

            let attrs = _element.get_attributes();
            for attr in attrs {
                log::info!("{:?}:{:?}", attr.0, attr.1);
            }

            log::info!("controltype:{_type}");
            log::info!("left:{_left} top:{_top} width:{_width} height:{_height}");
            log::info!("Elapsed time: {:?}", duration);
            log::info!("-----分割线------");

            //_automation.build_selector(_element);
        }
    }
}
