use crate::rpa_engine::rpa_core::error::Result;
use crate::rpa_engine::rpa_core::rect::MagicRect;
use std::collections::HashMap;

pub trait Element  {
    fn get_name(&self) -> Result<String>;
    fn get_classname(&self) -> Result<String>;
    fn get_bounding_rectangle(&self) -> Result<MagicRect>;
    fn get_control_type(&self) -> Result<String>;
    fn get_index(&self) -> Result<i32>;

    fn is_enabled(&self) -> Result<bool>;
    fn is_focus(&self) -> Result<bool>;
    fn is_edit(&self) -> Result<bool>;
    fn input(&self) -> Result<bool>;

    fn get_checked(&self) -> Result<bool>;
    fn set_check(&self) -> Result<bool>;
    fn get_text(&self) -> Result<String>;
    fn set_text(&self) -> Result<bool>;
    fn get_value(&self) -> Result<String>;

    fn get_select(&self) -> Result<Vec<String>>;
    fn set_select(&self) -> Result<bool>;
    fn event_click(&self) -> Result<bool>;
    fn get_attributes(&self) -> Result<HashMap<String, String>>;
    fn get_attribute(&self, _attribute_name: &str) -> Result<String>;
    fn set_attribute(&self, _attribute_name: &str, _attribute_value: &str) -> Result<bool>;

    fn get_parent(&self) -> Result<Box<dyn Element>>;
    fn get_children(&self) -> Result<Vec<Box<dyn Element>>>;
    fn get_descendants(&self) -> Result<Vec<Box<dyn Element>>>;
}
