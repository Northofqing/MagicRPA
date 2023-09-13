use crate::rpa_engine::rpa_automation::rpa_element::Element;
use crate::rpa_engine::rpa_automation::rpa_enum::TargeFramework;
use crate::rpa_engine::rpa_automation::rpa_trace::Trace;
use crate::rpa_engine::rpa_common::selector::{Pattern, Props, SelectorProps};
use crate::rpa_engine::rpa_core::handle::MagicHandle;
use crate::rpa_engine::rpa_core::point::MagicPoint;
use uiautomation::types::Point;
use uiautomation::UIAutomation;

use crate::rpa_engine::rpa_core::error::Result;

use super::uiaelement::WinUIAElement;

pub struct Win32Automation {
    pub automation: UIAutomation,
    pub target_framework: TargeFramework,
}

impl Win32Automation {
    pub fn new() -> Win32Automation {
        Self {
            automation: UIAutomation::new().unwrap(),
            target_framework: TargeFramework::UIA,
        }
    }
    // pub fn build_selector(name: String) {}
    // pub fn build_props() {}
    // pub fn enable_msaa() {}
    // pub fn IsChromiumFrameWork() -> bool {
    //     let _className = chromiumWin.ClassName;
    //     return _className.Contains("RenderWidgetHostHWND")
    //         || _className.Contains("WidgetWin")
    //         || _className.Contains("Intermediate D3D Window");
    // }
    pub fn recursion(_point: MagicPoint, element: Box<dyn Element>) -> Box<dyn Element> {
        let children = element.get_children().unwrap();
        for child in children {
            let rect = child.get_bounding_rectangle().unwrap();
            let point = MagicPoint::get_cursor_pos();
            if rect.contain(point) {
                return Self::recursion(_point, child);
            }
        }
        return element;
    }
    // pub fn catch_recursion(
    //     _point: MagicPoint,
    //     element: Box<dyn Element>,
    // ) -> Vec<Box<dyn Element>> {
    //     let mut _catch_elements: Vec<Box<dyn Element>> = Vec::new();
    //     let children = element.get_children();
    //     for child in children {
    //         let rect = child.get_bounding_rectangle();
    //         let point = MagicPoint::get_cursor_pos();
    //         if rect.contain(point) {
    //             _catch_elements.push(child);
    //         }
    //     }
    //     return _catch_elements;
    // }
}
impl Trace for Win32Automation {
    fn is_match(&self) -> Result<bool> {
        return Ok(true);
    }
    fn trace_element_point(&mut self, point: MagicPoint) -> Result<Box<dyn Element>> {
        self.target_framework = TargeFramework::MSAA;
        let uia_point = Point::new(point.x, point.y);
        match self.automation.element_from_point(uia_point) {
            Ok(element) => {
                let mut _uia_element = WinUIAElement::update_element(element);
                return Ok(Self::recursion(point, Box::new(_uia_element)));
            }
            Err(_) => {
                return Ok(Box::new(WinUIAElement::new()));
            }
        };
        // self.target_framework = TargeFramework::MSAA;
        // let uia_point = Point::new(point.x, point.y);
        // let ele = match self.automation.element_from_point(uia_point) {
        //     Ok(element) => element,
        //     Err(_) => {
        //         return Box::new(WinUIAElement::new());
        //     }
        // };

        // let mut _uia_element = WinUIAElement::update_element(ele);
        // let _parent = _uia_element.get_parent();
        // let _children = _parent.get_children();

        // let mut _catch_elements: Vec<Box<dyn Element>> = Vec::new();
        // for _ele in _children {
        //     let results = _ele.get_children();
        //     for element in results {
        //         let rect = element.get_bounding_rectangle();
        //         let point = MagicPoint::get_cursor_pos();
        //         if rect.contain(point) {
        //             match self.automation.element_from_point(uia_point) {
        //                 Ok(element) => {
        //                     let _box_win_element = Box::new(WinUIAElement::update_element(element));
        //                     _catch_elements.push(_box_win_element);
        //                 }
        //                 Err(_) => continue,
        //             };
        //         }
        //     }
        // }

        // _catch_elements.sort_by(|win_ele1, win_ele2| {
        //     win_ele1
        //         .get_bounding_rectangle()
        //         .area()
        //         .partial_cmp(&win_ele2.get_bounding_rectangle().area())
        //         .unwrap()
        // });
        // let _count = _catch_elements.len();
        // if _count > 0 {
        //     for item in _catch_elements {
        //         return item;
        //     }
        // }
        // return Box::new(_uia_element);
    }
    fn trace_element_handle(&self, handle: MagicHandle) -> Result<Box<dyn Element>> {
        let mut _ele = match self.automation.element_from_handle(handle.from()) {
            Ok(element) => element,
            Err(_) => {
                return Ok(Box::new(WinUIAElement::new()));
            }
        };
        return Ok(Box::new(WinUIAElement::update_element(_ele)));
    }
    fn trace_element_focus(&self) -> Result<Box<dyn Element>> {
        let ele = self.automation.get_focused_element().unwrap();

        return Ok(Box::new(WinUIAElement::update_element(ele)));
    }
    fn query_selector(&self, _selector: Vec<String>) -> Result<Box<dyn Element>> {
        let uia_point = Point::new(1, 1);
        let ele = self.automation.element_from_point(uia_point).unwrap();
        return Ok(Box::new(WinUIAElement::update_element(ele)));
    }
    fn query_selector_all(&self, _selector: Vec<String>) -> Result<Vec<Box<dyn Element>>> {
        let uia_point = Point::new(1, 1);
        let ele = self.automation.element_from_point(uia_point).unwrap();
        let mut _uia_element = WinUIAElement::update_element(ele);
        let result = _uia_element.get_children().unwrap();
        return Ok(result);
    }
    //fn query_selector_all(selector: Vec<String>) -> Vec<dyn Element> {}
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
    fn build_selector(&self, _element: Box<dyn Element>) -> Result<Vec<SelectorProps>> {
        let mut element = _element;
        let mut _selectors: Vec<SelectorProps> = Vec::new();

        loop {
            let _attrs = element.get_attributes();
            let mut props: Vec<Props> = Vec::new();

            if let Ok(name) = element.get_name() {
                if name != "" {
                    let prop = Props::new(String::from("name"), name, true, Pattern::EQUAL);
                    props.push(prop);
                }
            }

            if let Ok(classname) = element.get_classname() {
                if classname != "" {
                    let prop =
                        Props::new(String::from("classname"), classname, true, Pattern::EQUAL);
                    props.push(prop);
                }
            }

            if let Ok(text) = element.get_text() {
                if text != "" {
                    let prop = Props::new(String::from("text"), text, true, Pattern::EQUAL);
                    props.push(prop);
                }
            }

            if let Ok(index) = element.get_index() {
                if index > -1 {
                    let prop = Props::new(
                        String::from("index"),
                        index.to_string(),
                        true,
                        Pattern::EQUAL,
                    );
                    props.push(prop);
                }
            }

            let name = format!("{:?}", TargeFramework::UIA);
            let element_type = element.get_control_type().unwrap();
            let selector = SelectorProps::new(name, element_type, props, true);
            _selectors.push(selector);

            element = match element.get_parent() {
                Ok(e) => e,
                Err(_) => break,
            }
        }
        _selectors.reverse();
        log::info!("{:#?}", _selectors);
        return Ok(_selectors);
    }
}
