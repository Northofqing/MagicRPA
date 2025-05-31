use crate::rpa_engine::{
    rpa_automation::rpa_win32::win32automation::UIFramework, rpa_core::rect::MagicRect,
};

pub struct TraceData {
    pub bounding: MagicRect,
    info: String,
    ui_framework: UIFramework,
    process_id: u32,
    class_name: String,
}
impl TraceData {
    pub fn new(bounding: MagicRect, info: String, ui_framework: UIFramework) -> Self {
        Self {
            bounding: bounding,
            info: info,
            ui_framework: ui_framework,
            process_id: 0,
            class_name: "".to_string(),
        }
    }
}
