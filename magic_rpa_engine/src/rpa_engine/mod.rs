pub mod rpa_core {
    pub mod pinvoke;
    pub mod point;
    pub mod rect;
}
pub mod rpa_common{
    pub mod selector;
}
pub fn get_uiautomation_point() -> uiautomation::types::Point {
    return rpa_core::pinvoke::get_uiautomation_point();
}
pub mod rpa_automation;
// pub mod rpa_win32 {
//     pub mod uiaelement;
//     pub mod win32automation;
// }
