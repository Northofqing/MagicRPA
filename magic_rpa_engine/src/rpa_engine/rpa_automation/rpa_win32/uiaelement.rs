use uiautomation::controls::ControlType;
use uiautomation::types::Rect;
use uiautomation::UIAutomation;
use uiautomation::UIElement;

use crate::rpa_engine::rpa_core;
pub struct WinUIAElement {
    pub element: UIElement,
    pub automation: UIAutomation,
}
impl WinUIAElement {
    pub fn new(element: UIElement) -> Self {
        WinUIAElement {
            element,
            automation: UIAutomation::new().unwrap(),
        }
    }

    pub fn element_from_handle(&mut self, hwnd: uiautomation::types::Handle) -> WinUIAElement {
        let resut = self.automation.element_from_handle(hwnd);
        self.element = match resut {
            Ok(element) => element,
            Err(error) => panic!("not found element: {:?}", error),
        };
        let win_uia_element = WinUIAElement::new(self.element.clone());
        return win_uia_element;
    }
    pub fn trace_point(&mut self, point: uiautomation::types::Point) -> WinUIAElement {
        let resut = self.element_from_point(point);
        let  win_uia_element = WinUIAElement::new(resut);
        //win_uia_element = Self::element_traversal(win_uia_element, point);
        return win_uia_element;
    }
    pub fn element_from_point(&mut self, point: uiautomation::types::Point) -> UIElement {
        let resut = self.automation.element_from_point(point);
        self.element = match resut {
            Ok(element) => element,
            Err(error) => panic!("element from point not found element: {:?}", error),
        };
        return self.element.clone();
    }
    // pub fn element_from_point(&mut self, point: uiautomation::types::Point) -> WinUIAElement {
    //     let resut = self.automation.element_from_point(point);
    //     self.element = match resut {
    //         Ok(element) => element,
    //         Err(error) => panic!("not found element: {:?}", error),
    //     };
    //     let mut win_uia_element = WinUIAElement::new(self.element.clone());
    //     return win_uia_element;
    // }
    fn element_traversal(
        element: WinUIAElement,
        point: uiautomation::types::Point,
    ) -> WinUIAElement {
        let children = element.get_children();
        for child in children {
            let _rect = child.get_bounding_rectangle();
            let magic_rect = rpa_core::rect::MagicRect::get_rect(&_rect);
            let magic_point = rpa_core::point::MagicPoint::get_point(point);
            if magic_rect.contain(magic_point) {
                return Self::element_traversal(child, point);
            }
        }
        return element;
    }
    pub fn get_parent(&self) -> WinUIAElement {
        let walker = self.automation.get_raw_view_walker().unwrap();
        let parent = match walker.get_parent(&self.element) {
            Ok(element) => element,
            Err(error) => panic!("not found element: {:?}", error),
        };
        let win_uia_element = WinUIAElement::new(parent.clone());
        return win_uia_element;
    }
    pub fn get_children(&self) -> Vec<WinUIAElement> {
        let walker = self.automation.get_raw_view_walker().unwrap();
        let mut child = match walker.get_first_child(&self.element) {
            Ok(element) => element,
            Err(error) => panic!("get children not found element: {:?}", error),
        };
        let mut children = Vec::new();
        children.push(WinUIAElement::new(child.clone()));
        loop {
            child = match walker.get_next_sibling(&child) {
                Ok(element) => element,
                Err(error) => panic!("get next sibling not found element: {:?}", error),
            };
            break;
        }
        children.push(WinUIAElement::new(child.clone()));
        return children;
    }
    pub fn get_name(&self) -> String {
        let result = &self.element.get_name().unwrap();
        return result.clone();
    }
    pub fn get_classname(&self) -> String {
        let result = &self.element.get_classname().unwrap();
        return result.clone();
    }
    pub fn get_bounding_rectangle(&self) -> Rect {
        let result = &self.element.get_bounding_rectangle().unwrap();
        return result.clone();
    }
    pub fn get_control_type(&self) -> ControlType {
        let result = &self.element.get_control_type().unwrap();
        return result.clone();
    }
    pub fn is_enabled(&self) -> bool {
        let result = &self.element.is_enabled().unwrap();
        return result.clone();
    }

    // pub fn is_focus(&self) -> bool {

    // }
    // pub fn is_edit(&self) -> bool {
    //     return true;
    // }
    // pub fn is_checked(&self) -> bool {
    //     return true;
    // }
    // pub fn set_check(&self) -> bool {
    //     return true;
    // }

    // pub fn get_text(&self) -> &Result<String, uiautomation::Error>{}
    // pub fn set_text(&self) -> &Result<String, uiautomation::Error>{}
    // pub fn get_index(&self) -> i32 {
    //     return  1;
    // }

    // pub fn get_select(&self) -> Vect<&str> {

    // }
    // pub fn set_select(&self) -> Vect<&str> {

    // }
    // pub fn click(&self) -> bool {
    //     &self.element.click();
    //     return true;
    // }
    // pub fn get_attribute(&self, name: &str) -> Vect<&str> {
    //     return &self.element.get_attribute();
    // }
}
