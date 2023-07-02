use crate::rpa_engine::get_uiautomation_point;
use crate::rpa_engine::rpa_common::selector::{self, SelectorProps};
use std::time::Instant; 
pub struct Inspector {}
impl Inspector {
    pub fn inspect_element_point() {
        loop {
            // 开始计时
            let start = Instant::now();
            let point = get_uiautomation_point();
            let _automation =
                crate::rpa_engine::rpa_automation::rpa_win32::win32automation::Win32Automation::new(
                );
            let _element = _automation.trace(point);
            let _name = _element.get_name();
            let _classname = _element.get_classname();
            let _rect = _element.get_bounding_rectangle();
            let _left = _rect.get_left();
            let _top = _rect.get_top();
            let _width = _rect.get_width();
            let _height = _rect.get_height();
            let _type = _element.get_control_type();
            // 结束计时并计算经过的时间
            let duration = start.elapsed();
            println!("name:{_name} classname:{_classname} left:{_left} top:{_top} width:{_width} height:{_height}");
            println!("Elapsed time: {:?}", duration);
        }
    }
    pub fn build_selector(
        element: crate::rpa_engine::rpa_automation::rpa_win32::win32automation::Win32Automation,
    ) -> Vec<SelectorProps> {
        return vec![];
    }
    pub fn query_selector_all() {}
}
