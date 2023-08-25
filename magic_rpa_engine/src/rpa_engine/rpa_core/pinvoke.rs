extern crate winapi;
use winapi::shared::windef::{HWND, POINT};

pub fn window_from_point() -> HWND {
    use winapi::um::winuser::WindowFromPoint;
    let pt = POINT { x: -1, y: -1 };
    let ret = unsafe {
        let hwd = WindowFromPoint(pt);
        if hwd.is_null() {
            println!("No window found at the specified point.");
        }
        return hwd.clone();
    };
}
pub fn get_cursor_pos() -> Option<(i32, i32)> {
    use winapi::um::winuser::GetCursorPos;

    let mut pt = POINT { x: -1, y: -1 };
    let ret = unsafe { GetCursorPos(&mut pt) };
    if ret != 1 || pt.x == -1 && pt.y == -1 {
        None
    } else {
        Some((pt.x, pt.y))
    }
}
