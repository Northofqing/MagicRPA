use crate::rpa_engine::rpa_core::rect::MagicRect;

use crate::rpa_engine::rpa_automation::rpa_element::RPAElement;
use std::collections::HashMap;
pub struct JavaElement {}

impl RPAElement for JavaElement {
    fn get_name(&self) -> String {
        return String::new();
    }
    fn get_classname(&self) -> String {
        return String::new();
    }
    fn get_bounding_rectangle(&self) -> MagicRect {
        return MagicRect {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
    }
    fn get_control_type(&self) -> String {
        return String::new();
    }
    fn is_enabled(&self) -> bool {
        return true;
    }
    fn is_focus(&self) -> bool {
        return true;
    }
    fn is_edit(&self) -> bool {
        return true;
    }

    fn get_checked(&self) -> bool {
        return true;
    }
    fn set_check(&self) -> bool {
        return true;
    }
    fn get_text(&self) -> String {
        return String::new();
    }
    fn set_text(&self) -> String {
        return String::new();
    }

    fn get_select(&self) -> Vec<String> {
        let selects: Vec<String> = Vec::new();
        return selects;
    }
    fn set_select(&self) -> Vec<String> {
        let selects: Vec<String> = Vec::new();
        return selects;
    }
    fn event_click(&self) -> bool {
        return true;
    }
    fn get_attributes(&self) -> HashMap<String, String> {
        let mut map_attributes: HashMap<String, String> = HashMap::new();
        return map_attributes;
    }
    fn get_attribute(&self, attribute_name: &str) -> String {
        return String::from("");
    }
    fn set_attribute(&self, attribute_name: &str, attribute_value: &str) -> bool {
        return true;
    }

    fn get_parent(&self) -> Box<dyn RPAElement> {
        let win_element = JavaElement {};
        return Box::new(win_element);
    }
    fn get_children(&self) -> Vec<Box<dyn RPAElement>> {
        let mut win_elements = Vec::new();

        let win_element = JavaElement {};
        return win_elements;
    }
    fn get_descendants(&self) -> Vec<Box<dyn RPAElement>> {
        let mut win_elements = Vec::new();

        let win_element = JavaElement {};
        return win_elements;
    }
}
