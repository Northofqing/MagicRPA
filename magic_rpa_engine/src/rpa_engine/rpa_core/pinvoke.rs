extern crate winapi;

// fn set_cursor_pos(x: i32, y: i32) -> bool {
//     use winapi::um::winuser::SetCursorPos;
//     let ret = unsafe { SetCursorPos(x, y) };
//     ret == 1
// }

fn get_cursor_pos() -> Option<(i32, i32)> {
    use winapi::shared::windef::POINT;
    use winapi::um::winuser::GetCursorPos;

    let mut pt = POINT { x: -1, y: -1 };
    let ret = unsafe { GetCursorPos(&mut pt) };
    if ret != 1 || pt.x == -1 && pt.y == -1 {
        None
    } else {
        Some((pt.x, pt.y))
    }
}
pub fn get_uiautomation_point() -> uiautomation::types::Point {
    let point_option = get_cursor_pos().unwrap();
    let point = uiautomation::types::Point::new(point_option.0, point_option.1);
    return point;
}
