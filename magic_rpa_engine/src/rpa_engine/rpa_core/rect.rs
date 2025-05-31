use crate::rpa_engine;

use super::point::MagicPoint;
#[derive(Debug)]
pub struct MagicRect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl MagicRect {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> MagicRect {
        return MagicRect {
            x: x,
            y: y,
            width: width,
            height: height,
        };
    }
    // pub fn get_rect(rect: &uiautomation::types::Rect) -> MagicRect {
    //     return MagicRect {
    //         x: rect.get_left(),
    //         y: rect.get_top(),
    //         width: rect.get_right() - rect.get_left(),
    //         height: rect.get_bottom() - rect.get_top(),
    //     };
    // }
    pub fn contain(&self, point: rpa_engine::rpa_core::point::MagicPoint) -> bool {
        if self.x <= point.x
            && self.y <= point.y
            && self.x + self.width >= point.x
            && self.y + self.height >= point.y
        {
            return true;
        } else {
            return false;
        }
    }
    pub fn center(&self) -> MagicPoint {
        MagicPoint::new(&self.x + &self.width / 2, &self.y + &self.height / 2)
    }
    pub fn area(&self) -> i32 {
        return self.width * self.height;
    }
}
