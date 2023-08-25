use crate::rpa_engine::rpa_core::rect::MagicRect;
use std::collections::HashMap;
pub trait RPAElement {
    fn get_name(&self) -> String;
    fn get_classname(&self) -> String;
    fn get_bounding_rectangle(&self) -> MagicRect;
    fn get_control_type(&self) -> String;

    fn is_enabled(&self) -> bool;
    fn is_focus(&self) -> bool;
    fn is_edit(&self) -> bool;

    fn get_checked(&self) -> bool;
    fn set_check(&self) -> bool;
    fn get_text(&self) -> String;
    fn set_text(&self) -> String;

    fn get_select(&self) -> Vec<String>;
    fn set_select(&self) -> Vec<String>;
    fn event_click(&self) -> bool;
    fn get_attributes(&self) -> HashMap<String, String>;
    fn get_attribute(&self, attribute_name: &str) -> String;
    fn set_attribute(&self, attribute_name: &str,attribute_value: &str) -> bool;

    fn get_parent(&self) -> Box<dyn RPAElement>;
    fn get_children(&self) -> Vec<Box<dyn RPAElement>>;
    fn get_descendants(&self) -> Vec<Box<dyn RPAElement>>;
}
