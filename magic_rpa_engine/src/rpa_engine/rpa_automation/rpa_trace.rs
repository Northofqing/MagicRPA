use crate::rpa_engine::rpa_automation::rpa_element::Element;
use crate::rpa_engine::rpa_common::selector::Selector;
use crate::rpa_engine::rpa_core::error::Result;
use crate::rpa_engine::rpa_core::handle::MagicHandle;
use crate::rpa_engine::rpa_core::point::MagicPoint;
pub trait Trace {
    fn is_match(&self) -> Result<bool>;
    fn trace_element_point(&mut self, _point: MagicPoint) -> Result<Box<dyn Element>>;
    fn trace_element_handle(&self, _handle: MagicHandle) -> Result<Box<dyn Element>>;
    fn trace_element_focus(&self) -> Result<Box<dyn Element>>;
    fn query_selector(&self, _selector: Vec<String>) -> Result<Box<dyn Element>>;
    fn query_selector_all(&self, _selector: Vec<String>) -> Result<Vec<Box<dyn Element>>>;
    fn build_selector(&self, _element: Box<dyn Element>) -> Result<Vec<Selector>>;
}
