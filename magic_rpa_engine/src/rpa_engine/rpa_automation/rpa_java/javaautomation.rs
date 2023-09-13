// use crate::rpa_engine::rpa_automation::rpa_element::RPAElement;
// use crate::rpa_engine::rpa_automation::rpa_enum::TargeFramework;
// use crate::rpa_engine::rpa_automation::rpa_trace::RPATrace;
// use crate::rpa_engine::rpa_common::selector::SelectorProps;
// use crate::rpa_engine::rpa_core::handle::MagicHandle;
// use crate::rpa_engine::rpa_core::point::MagicPoint;

// use super::javaelement::JavaElement;

// use crate::rpa_engine::rpa_core::error::Result;

// pub struct JavaAutomation {
//     target_framework: TargeFramework,
// }

// impl RPATrace for JavaAutomation {
//     fn is_match(&self) -> Result<bool> {
//         return Ok(true);
//     }
//     fn trace_element_point(&mut self, _point: MagicPoint) -> Result<Box<dyn RPAElement>> {
//         self.target_framework = TargeFramework::JAVA;
//         let javaelement = JavaElement {};
//         return Box::new(javaelement);
//     }
//     fn trace_element_handle(&self, _handle: MagicHandle) -> Result<Box<dyn RPAElement>> {
//         let javaelement = JavaElement {};
//         return Box::new(javaelement);
//     }
//     fn trace_element_focus(&self) -> Result<Box<dyn RPAElement>> {
//         let javaelement = JavaElement {};
//         return Box::new(javaelement);
//     }
//     fn query_selector(&self, _selector: Vec<String>) -> Result<Box<dyn RPAElement>> {
//         let javaelement = JavaElement {};
//         return Box::new(javaelement);
//     }
//     fn query_selector_all(&self, _selector: Vec<String>) -> Result<Vec<Box<dyn RPAElement>>> {
//         let javaelement = JavaElement {};
//         let result = javaelement.get_children();
//         return result;
//     }
//     fn build_selector(&self, _element: Box<dyn RPAElement>) -> Result<Vec<SelectorProps>> {
//         let _selectors: Vec<SelectorProps> = Vec::new();
//         let _attrs = _element.get_attributes();
//         let mut _parent = _element.get_parent();
//         loop {
//             _parent.get_attributes();
//             _parent = _parent.get_parent();
//             break;
//         }

//         return _selectors;
//     }
// }
