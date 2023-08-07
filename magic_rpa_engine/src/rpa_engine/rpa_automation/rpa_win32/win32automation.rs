use crate::rpa_engine::rpa_automation::rpa_element::RPAElement;
use crate::rpa_engine::rpa_automation::rpa_trace::RPATrace;
use crate::rpa_engine::rpa_common::selector::SelectorProps;
use crate::rpa_engine::rpa_core::handle::MagicHandle;
use crate::rpa_engine::rpa_core::point::MagicPoint;
use uiautomation::types::Point;
use uiautomation::UIAutomation;

use super::uiaelement::WinUIAElement;

mod magic_uia_element {
    pub use crate::rpa_engine::rpa_automation::rpa_win32::uiaelement;
    pub use crate::rpa_engine::rpa_automation::rpa_win32::win32automation;
}

#[derive(Debug)]
pub struct Win32Automation {
    pub automation: UIAutomation,
}

impl RPATrace for Win32Automation {
    fn is_match(&self) -> bool {
        return bool::from(true);
    }
    fn trace_element_point(&self, point: MagicPoint) -> Box<dyn RPAElement> {
        let uia_point = Point::new(point.x, point.y);
        let mut ele = match self.automation.element_from_point(uia_point) {
            Ok(element) => element,
            Err(_) => {
                return Box::new(WinUIAElement {
                    automation: self.automation.clone(),
                    element: self.automation.get_root_element().unwrap(),
                });
            }
        };
        let mut win_uia_element = WinUIAElement {
            automation: self.automation.clone(),
            element: ele,
        };
        WinUIAElement::test();
        let result = WinUIAElement::get_children(&win_uia_element);
        let len = result.len();
        if len > 1 {
            for element in result {
                let rect = element.get_bounding_rectangle();
                let point = MagicPoint::get_cursor_pos();
                if rect.contain(point) {
                    ele = match self.automation.element_from_point(uia_point) {
                        Ok(element) => element,
                        Err(_) => continue,
                    };
                    win_uia_element = WinUIAElement {
                        automation: self.automation.clone(),
                        element: ele,
                    };
                    return Box::new(win_uia_element);
                }
            }
        }
        return Box::new(win_uia_element);
    }
    fn trace_element_handle(&self, handle: MagicHandle) -> Box<dyn RPAElement> {
        let mut ele = match self.automation.element_from_handle(handle.from()) {
            Ok(element) => element,
            Err(_) => {
                return Box::new(WinUIAElement {
                    automation: self.automation.clone(),
                    element: self.automation.get_root_element().unwrap(),
                });
            }
        };
        let mut win_uia_element = WinUIAElement {
            automation: self.automation.clone(),
            element: ele,
        };
        return Box::new(win_uia_element);
    }
    fn trace_element_focus(&self) -> Box<dyn RPAElement> {
        let uia_point = Point::new(1, 1);
        let ele = self.automation.element_from_point(uia_point).unwrap();
        let mut win_uia_element = WinUIAElement {
            automation: self.automation.clone(),
            element: ele,
        };
        return Box::new(win_uia_element);
    }
    fn query_selector(&self, selector: Vec<String>) -> Box<dyn RPAElement> {
        let uia_point = Point::new(1, 1);
        let ele = self.automation.element_from_point(uia_point).unwrap();
        let mut win_uia_element = WinUIAElement {
            automation: self.automation.clone(),
            element: ele,
        };
        return Box::new(win_uia_element);
    }
    fn query_selector_all(&self, selector: Vec<String>) -> Vec<Box<dyn RPAElement>> {
        let uia_point = Point::new(1, 1);
        let ele = self.automation.element_from_point(uia_point).unwrap();
        let mut win_uia_element = WinUIAElement {
            automation: self.automation.clone(),
            element: ele,
        };
        let result = win_uia_element.get_children();
        return result;
    }
    //fn query_selector_all(selector: Vec<String>) -> Vec<dyn RPAElement> {}
    // pub fn new() -> Self {
    //     Win32Automation {
    //         automation: UIAutomation::new().unwrap(),
    //     }
    // }
    // pub fn trace(&self, point: uiautomation::types::Point) -> uiaelement::WinUIAElement {
    //     let ele = self.automation.get_root_element().unwrap();
    //     let mut win_uia_element = uiaelement::WinUIAElement {
    //         element: ele,
    //         automation: self.automation.clone(),
    //     };
    //     let result = rpa_engine::rpa_automation::rpa_win32::uiaelement::WinUIAElement::trace_point(
    //         &mut win_uia_element,
    //         point,
    //     );
    //     return result;
    // }
    fn build_selector(&self, element: Box<dyn RPAElement>) -> Vec<SelectorProps> {
        let selectors: Vec<SelectorProps> = Vec::new(); 
        loop {}

        return selectors;
    }
}
