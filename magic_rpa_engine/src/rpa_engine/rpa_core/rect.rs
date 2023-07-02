use crate::rpa_engine;
#[warn(dead_code)]
 pub struct MagicRect {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}
impl MagicRect {
   pub fn get_rect(rect: &uiautomation::types::Rect) -> MagicRect {
       return  MagicRect {
            x: rect.get_left(),
            y: rect.get_top(),
            width: rect.get_right() - rect.get_left(),
            height: rect.get_bottom() - rect.get_top(),
        };
    }
    pub fn contain(&self, point:rpa_engine::rpa_core::point::MagicPoint) -> bool {
        if (self.x >= point.x
            && self.y >= point.y
            && self.x + self.width >= point.x
            && self.y + self.height >= point.y)
        {
            return true;
        } else {
            return false;
        }
    }
    pub fn area(&self) -> i32 {
        return self.width * self.height;
    }
}
