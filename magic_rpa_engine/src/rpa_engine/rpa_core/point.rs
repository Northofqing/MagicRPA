use super::pinvoke;

extern crate winapi;

pub struct MagicPoint {
    pub x: i32,
    pub y: i32,
}

impl MagicPoint {
    // pub fn new(x: i32, y: i32) -> MagicPoint {
    //     return MagicPoint { x: x, y: y };
    // }
    pub fn get_cursor_pos() -> MagicPoint {
        let _point = pinvoke::get_cursor_pos();
        let _result = _point.unwrap();
        return MagicPoint {
            x: _result.0,
            y: _result.1,
        };
    }
    // pub fn get_point(point: uiautomation::types::Point) -> MagicPoint {
    //     return MagicPoint {
    //         x: point.get_x(),
    //         y: point.get_y(),
    //     };
    // }
}
