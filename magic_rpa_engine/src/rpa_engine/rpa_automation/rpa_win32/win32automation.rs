use uiautomation::UIAutomation;
//use uiautomation::UITreeWalker;

use crate::rpa_engine;

use super::uiaelement;

mod magic_uia_element {
    pub use crate::rpa_engine::rpa_automation::rpa_win32::uiaelement;
    pub use crate::rpa_engine::rpa_automation::rpa_win32::win32automation;
}

#[derive(Debug)]
pub struct Win32Automation {
    automation: UIAutomation,
}

impl Win32Automation {
    pub fn new() -> Self {
        Win32Automation {
            automation: UIAutomation::new().unwrap(),
        }
    }
    pub fn trace(&self, point: uiautomation::types::Point) -> uiaelement::WinUIAElement {
        let ele = self.automation.get_root_element().unwrap();
        let mut win_uia_element = uiaelement::WinUIAElement {
            element: ele,
            automation: self.automation.clone(),
        };
        let result =
            rpa_engine::rpa_automation::rpa_win32::uiaelement::WinUIAElement::trace_point(
                &mut win_uia_element,
                point,
            );
        return result;
    } 
}
