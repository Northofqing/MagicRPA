use super::pinvoke;

extern crate winapi;
#[warn(dead_code)]
pub struct MagicPoint {
    pub x: i32,
    pub y: i32,
}
impl MagicPoint {
    pub fn new(x: i32, y: i32) -> MagicPoint {
        return MagicPoint { x: x, y: y };
    }
    pub fn get_cursor_pos() -> MagicPoint {
        let point = pinvoke::get_cursor_pos();
        let p = point.unwrap();
        return MagicPoint { x: p.0, y: p.1 };
    }
    pub fn get_point(point: uiautomation::types::Point) -> MagicPoint {
        return MagicPoint {
            x: point.get_x(),
            y: point.get_y(),
        };
    }
}
