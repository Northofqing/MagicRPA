use crate::rpa_engine::rpa_automation::rpa_element::RPAElement;
use crate::rpa_engine::rpa_automation::rpa_trace::RPATrace;
use crate::rpa_engine::rpa_automation::rpa_enum::targeFramework;
use crate::rpa_engine::rpa_common::selector::SelectorProps;
use crate::rpa_engine::rpa_core::handle::MagicHandle;
use crate::rpa_engine::rpa_core::point::MagicPoint;

use super::javaelement::JavaElement;

pub struct JavaAutomation {
    pub target_framework: targeFramework,
}

impl RPATrace for JavaAutomation {
    fn is_match(&self) -> bool {
        return true;
    }
    fn trace_element_point(&mut self, point: MagicPoint) -> Box<dyn RPAElement> {
        self.target_framework = targeFramework::java;
        let javaelement = JavaElement {};
        return Box::new(javaelement);
    }
    fn trace_element_handle(&self, handle: MagicHandle) -> Box<dyn RPAElement> {
        let javaelement = JavaElement {};
        return Box::new(javaelement);
    }
    fn trace_element_focus(&self) -> Box<dyn RPAElement> {
        let javaelement = JavaElement {};
        return Box::new(javaelement);
    }
    fn query_selector(&self, selector: Vec<String>) -> Box<dyn RPAElement> {
        let javaelement = JavaElement {};
        return Box::new(javaelement);
    }
    fn query_selector_all(&self, selector: Vec<String>) -> Vec<Box<dyn RPAElement>> {
        let javaelement = JavaElement {};
        let result = javaelement.get_children();
        return result;
    }
    fn build_selector(&self, element: Box<dyn RPAElement>) -> Vec<SelectorProps> {
        let selectors: Vec<SelectorProps> = Vec::new();
        loop {}

        return selectors;
    }
}
