use crate::rpa_engine::rpa_core::handle::MagicHandle;
use crate::rpa_engine::rpa_core::point::MagicPoint;
use uiautomation::{types::{Handle, Point}, UIAutomation,UIElement};
pub struct UIAHelper {
   pub automation: UIAutomation,
}
impl UIAHelper {
    pub fn new() -> Self {
        Self {
            automation: UIAutomation::new().unwrap(),
        }
    }
    pub fn element_from_handle(&self, magic_handle: MagicHandle) -> UIElement {
        let handle = Handle::from(magic_handle.hwnd.0 as isize);
        self.automation.element_from_handle(handle).unwrap()
    }
    pub fn element_from_point(&self, magic_Point: MagicPoint) -> UIElement {
        let point = Point::new(magic_Point.x, magic_Point.y);
        self.automation.element_from_point(point).unwrap()
    }
}
