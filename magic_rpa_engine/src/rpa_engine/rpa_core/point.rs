extern crate winapi;
#[warn(dead_code)]
pub struct MagicPoint {
    pub x: i32,
    pub y: i32,
}
impl MagicPoint {
    pub fn new() -> MagicPoint {
        return MagicPoint { x: 1, y: 1 };
    }
    pub fn get_point(point: uiautomation::types::Point) -> MagicPoint {
        return MagicPoint { x: point.get_x(), y: point.get_y()};
    }
}
