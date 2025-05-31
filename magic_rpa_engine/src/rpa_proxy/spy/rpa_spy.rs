use std::any::Any;

use super::{element_info::ElementInfo, spy_context::SpyContext, trace_data::TraceData};

pub trait ISpy {
    fn is_match(&self, context: &SpyContext) -> bool;
    fn trace(&self, context: &SpyContext) -> Option<TraceData>;
    fn trace_focus(&self, context: &SpyContext) -> Option<TraceData>;
    fn resolve_element(&self, context: &SpyContext) -> Option<ElementInfo>;
    fn resolve_focused_element(&self) -> Option<ElementInfo>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}