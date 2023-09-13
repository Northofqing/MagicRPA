// use crate::rpa_engine::rpa_core::rect::MagicRect;

// use crate::rpa_engine::rpa_automation::rpa_element::RPAElement;
// use std::collections::HashMap;

// use crate::rpa_engine::rpa_core::error::Result;
// pub struct JavaElement {}

// impl RPAElement for JavaElement {
//     fn get_name(&self) -> Result<String> {
//         return Ok(String::new());
//     }
//     fn get_classname(&self) -> Result<String> {
//         return Ok(String::new());
//     }
//     fn get_bounding_rectangle(&self) -> Result<MagicRect> {
//         return Ok(MagicRect {
//             x: 0,
//             y: 0,
//             width: 0,
//             height: 0,
//         });
//     }
//     fn get_control_type(&self) -> Result<String> {
//         return Ok(String::new());
//     }
//     fn get_index(&self) -> Result<i32> {
//         return Ok(1);
//     }
//     fn is_enabled(&self) -> Result<bool> {
//         return Ok(true);
//     }
//     fn is_focus(&self) -> Result<bool> {
//         return Ok(true);
//     }
//     fn is_edit(&self) -> Result<bool> {
//         return Ok(true);
//     }

//     fn get_checked(&self) -> Result<bool> {
//         return Ok(true);
//     }
//     fn set_check(&self) -> Result<bool> {
//         return Ok(true);
//     }
//     fn get_text(&self) -> Result<String> {
//         return Ok(String::new());
//     }
//     fn set_text(&self) -> Result<bool> {
//         return Ok(true);
//     }

//     fn get_select(&self) -> Result<Vec<String>> {
//         let selects: Vec<String> = Vec::new();
//         return Ok(selects);
//     }
//     fn set_select(&self) -> Result<bool> {
//         return Ok(true);
//     }
//     fn event_click(&self) -> Result<bool> {
//         return Ok(true);
//     }
//     fn get_attributes(&self) -> Result<HashMap<String, String>> {
//         let map_attributes: HashMap<String, String> = HashMap::new();
//         return Ok(map_attributes);
//     }
//     fn get_attribute(&self, _attribute_name: &str) -> Result<String> {
//         return Ok(String::from(""));
//     }
//     fn set_attribute(&self, _attribute_name: &str, _attribute_value: &str) -> Result<bool> {
//         return Ok(true);
//     }

//     fn get_parent(&self) -> Result<Box<dyn RPAElement>> {
//         let win_element = JavaElement {};
//         return Ok(Box::new(win_element));
//     }
//     fn get_children(&self) -> Result<Vec<Box<dyn RPAElement>>> {
//         let win_elements = Vec::new();

//         let _win_element = JavaElement {};
//         return Ok(win_elements);
//     }
//     fn get_descendants(&self) -> Result<Vec<Box<dyn RPAElement>>> {
//         let win_elements = Vec::new();

//         let _win_element = JavaElement {};
//         return Ok(win_elements);
//     }
// }
