use crate::rpa_engine::rpa_automation::rpa_element::RPAElement;
use crate::rpa_engine::rpa_common::selector::SelectorProps;
use crate::rpa_engine::rpa_core::handle::MagicHandle;
use crate::rpa_engine::rpa_core::point::MagicPoint;
pub trait RPATrace {
    fn is_match(&self) -> bool;
    fn trace_element_point(&mut self, point: MagicPoint) -> Box<dyn RPAElement>;
    fn trace_element_handle(&self, handle: MagicHandle) -> Box<dyn RPAElement>;
    fn trace_element_focus(&self) -> Box<dyn RPAElement>;
    fn query_selector(&self, selector: Vec<String>) -> Box<dyn RPAElement>;
    fn query_selector_all(&self, selector: Vec<String>) -> Vec<Box<dyn RPAElement>>;
    fn build_selector(&self, element: Box<dyn RPAElement>) -> Vec<SelectorProps>;
}
