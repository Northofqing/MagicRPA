use uiautomation::UIAutomation;
use uiautomation::UIElement;

use super::uiaelement;
mod UIAElement{
    pub use crate::rpa_engine::rpa_win32::win32automation;
    pub use crate::rpa_engine::rpa_win32::uiaelement;
}

#[derive(Debug)]
pub struct Win32Automation {
    automation: UIAutomation,
}

impl Win32Automation {
    
    pub fn element_from_point(point: uiautomation::types::Point)  {
        let ele = UIAutomation::new().unwrap();
        let element = ele.element_from_point(point);
        let e = element.unwrap();
        let el = uiaelement::WinUIAElement::new(e,ele);
        el.get_name();
        el.get_classname();
        el.get_bounding_rectangle();

        
    }
    // pub fn element_from_handle(self, hwnd: uiautomation::types::Handle) -> WinUIAElement {
    //     let element = self.automation.element_from_handle(hwnd);
    //     return uia_element::WinUIAElement::new(element);
    // }
    //pub fn element_trace() -> UIAElement {}
}
