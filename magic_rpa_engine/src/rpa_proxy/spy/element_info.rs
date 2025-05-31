use crate::rpa_engine::{rpa_common::selector::Selector, rpa_core::rect::MagicRect};

pub struct ElementInfo {
    class_name: String,
    selector: Selector,
    bounding: MagicRect,
    element_id: String,
    text: String,
}