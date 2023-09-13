extern crate winapi;
use winapi::shared::minwindef::UINT;
use winapi::shared::windef::{HWND, POINT};
use winapi::um::winuser::ChildWindowFromPointEx;
use winapi::um::winuser::{CWP_ALL, CWP_SKIPDISABLED, CWP_SKIPINVISIBLE, CWP_SKIPTRANSPARENT};
// pub fn window_from_point() -> HWND {
//     use winapi::um::winuser::WindowFromPoint;
//     let pt = POINT { x: -1, y: -1 };
//     let ret = unsafe {
//         let hwd = WindowFromPoint(pt);
//         if hwd.is_null() {
//             println!("No window found at the specified point.");
//         }
//         return hwd.clone();
//     };
// }
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
// pub enum CwpFLAGS {
//     CWP_ALL,
//     // CWP_ALL = CWP_ALL,
//     // CWP_SKIPINVISIBLE = CWP_SKIPINVISIBLE,
//     // CWP_SKIPDISABLED = CWP_SKIPDISABLED,
//     // CWP_SKIPTRANSPARENT = CWP_SKIPTRANSPARENT,
// }
// pub fn child_window_from_point_ex(hwnd: HWND, pt: POINT, flags: CwpFLAGS) -> Option<HWND> {
//     let ret = unsafe { ChildWindowFromPointEx(hwnd, pt, flags.) };
//     return Some(ret);
// }
