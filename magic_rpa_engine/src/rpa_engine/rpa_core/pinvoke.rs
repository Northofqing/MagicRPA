use windows::{
    core::*,
    Win32::{
        Foundation::*,
        Graphics::{Dwm::*, Gdi::*},
        System::{Com::*, SystemInformation::*, Threading::*},
        UI::{Accessibility::*, Input::KeyboardAndMouse::*, WindowsAndMessaging::*},
    },
};

use super::point::MagicPoint;

pub fn dwm_get_window_attribute<T: Default>(
    hwnd: HWND,
    attrinute: i32,
) -> windows::core::Result<T> {
    let mut value = T::default();

    unsafe {
        DwmGetWindowAttribute(
            hwnd,
            DWMWINDOWATTRIBUTE(attrinute),
            &mut value as *mut T as *mut _,
            std::mem::size_of::<T>() as u32,
        )?;
    }
    Ok(value)
}

pub fn attach_thread_input(idattach: u32, idattachto: u32, fattach: bool) -> bool {
    unsafe { AttachThreadInput(idattach, idattachto, fattach).as_bool() }
}

pub fn get_current_threadId() -> u32 {
    unsafe { GetCurrentProcessId() }
}
pub fn get_foreground_window() -> HWND {
    unsafe { GetForegroundWindow() }
}
pub fn get_focus() -> HWND {
    unsafe { GetFocus() }
}
pub fn get_active_window() -> HWND {
    unsafe { GetActiveWindow() }
}

pub fn is_iconic(hwnd: HWND) -> bool {
    unsafe { IsIconic(hwnd).as_bool() }
}

/*
GWL_EXSTYLE
-20
Retrieves the extended window styles.
GWL_HINSTANCE
-6
Retrieves a handle to the application instance.
GWL_HWNDPAREMagic
-8
Retrieves a handle to the parent window, if any.
GWL_ID
-12
Retrieves the identifier of the window.
GWL_STYLE
-16
Retrieves the window styles.
GWL_USERDATA
-21
Retrieves the user data associated with the window. This data is intended for use by the application that created the window. Its value is initially zero.
GWL_WNDPROC
-4
Retrieves the address of the window procedure, or a handle representing the address of the window procedure. You must use the CallWindowProc function to call the window procedure.
*/
pub fn get_window_long(hwnd: HWND, index: i32) -> i32 {
    unsafe { GetWindowLongW(hwnd, WINDOW_LONG_PTR_INDEX(index)) }
}

pub fn get_client_rect(hwnd: HWND) -> Option<(i32, i32, i32, i32)> {
    unsafe {
        let mut rect = RECT::default();
        GetClientRect(hwnd, &mut rect);
        Some((rect.left, rect.top, rect.right, rect.bottom))
    }
}
pub fn get_window_rect(hwnd: HWND) -> Option<(i32, i32, i32, i32)> {
    unsafe {
        let mut rect = RECT::default();
        GetWindowRect(hwnd, &mut rect);
        Some((rect.left, rect.top, rect.right, rect.bottom))
    }
}
pub fn get_window_thread_processId(hwnd: HWND) -> u32 {
    unsafe {
        let mut process_id = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut process_id));
        process_id
    }
}

pub fn get_desktop_window() -> Option<HWND> {
    unsafe { Some(GetDesktopWindow()) }
}
/// GW_HWNDFIRST 0
/// GW_HWNDLAST 1
/// GW_HWNDNEXT 2
/// GW_HWNDPREV 3
/// GW_OWNER 4
/// GW_CHILD 5
/// GW_ENABLEDPOPUP 6
pub fn get_window(hwnd: HWND, flags: u32) -> Option<HWND> {
    match unsafe { GetWindow(hwnd, GET_WINDOW_CMD(flags)) } {
        Ok(hwnd) => Some(hwnd),
        Err(e) => Some(HWND::default()),
    }
}

pub fn is_window(hwnd: HWND) -> bool {
    unsafe { IsWindow(hwnd).as_bool() }
}

/// 返回父窗口 0
/// 返回根窗口（顶层窗口） 1
/// 返回根所有者窗口 2
pub fn get_ancestor(hwnd: HWND, flags: u32) -> Option<HWND> {
    unsafe {
        let ancestor = GetAncestor(hwnd, GET_ANCESTOR_FLAGS(flags));
        Some(ancestor)
    }
}

pub fn screen_to_client(hwnd: HWND, x: i32, y: i32) -> Result<(i32, i32)> {
    unsafe {
        let mut point = POINT { x, y };
        ScreenToClient(hwnd, &mut point);
        Ok((point.x, point.y))
    }
}

pub fn client_to_screen(hwnd: HWND, x: i32, y: i32) -> Result<(i32, i32)> {
    unsafe {
        let mut point = POINT { x, y };
        ClientToScreen(hwnd, &mut point);
        Ok((point.x, point.y))
    }
}
pub fn get_window_from_point(x: i32, y: i32) -> Option<HWND> {
    let point = POINT { x, y };
    let hwnd = unsafe { WindowFromPoint(point) };

    if hwnd == HWND::default() {
        None
    } else {
        Some(hwnd)
    }
}

pub fn get_child_window(hwnd: HWND) -> HWND {
    unsafe { GetWindow(hwnd, GW_CHILD).unwrap() }
}
pub fn get_next_window(hwnd: HWND) -> HWND {
    unsafe { GetWindow(hwnd, GW_HWNDNEXT).unwrap() }
}
// pub fn accessible_object_from_window(
//     hwnd: HWND,
//     dwid: u32,
//     riid: *const windows_core::GUID,
//     ppvobject: *mut *mut core::ffi::c_void,
// ) {
//    let  riid =  windows_core::GUID::from("{618736E0-3C3D-11CF-810C-00AA00389B71}");
//     unsafe {
//         AccessibleObjectFromWindow(hwnd, dwid, riid, ppvobject);
//     }
// }

pub fn get_accessible_object_from_window(hwnd: HWND) -> Option<IAccessible> {
    unsafe {
        let mut accessible: Option<IAccessible> = None;

        // 调用 AccessibleObjectFromWindow
        AccessibleObjectFromWindow(
            hwnd,
            OBJID_CLIENT.0 as u32, // dwObjectID
            &IAccessible::IID,     // riid
            &mut accessible as *mut _ as *mut *mut core::ffi::c_void,
        );

        accessible
    }
}
pub fn accessible_object_from_point(point: MagicPoint) {
    let result = unsafe {
        let mut accessible = None;
        let mut child = VARIANT::default();
        let mut p = POINT::default();
        p.x = point.x;
        p.y = point.y;
        match AccessibleObjectFromPoint(p, &mut accessible, &mut child) {
            Ok(()) => {
                if let Some(acc) = accessible {
                    // 创建空的 VARIANT
                    let v = VARIANT::default();

                    // 尝试获取名称
                    match acc.get_accName(&v) {
                        Ok(name) => println!("Name: {}", name),
                        Err(e) => println!("Error getting name: {:?}", e),
                    }

                    Ok(())
                } else {
                    println!("No accessible object found");
                    Ok(())
                }
            }
            Err(e) => Err(e),
        }
    };

    if let Err(e) = result {
        println!("Error: {:?}", e);
    }
}
/// 检查窗口是否可见
pub fn is_window_visible(hwnd: HWND) -> bool {
    unsafe { IsWindowVisible(hwnd).as_bool() }
}
pub fn is_window_enable(hwnd: HWND) -> bool {
    unsafe { IsWindowEnabled(hwnd).as_bool() }
}
pub fn send_message_timeout(
    hwnd: HWND,
    msg: u32,
    wparam: usize,
    lparam: isize,
    timeout_ms: u32,
) -> Result<usize> {
    let mut result: usize = 0;

    let flags = SEND_MESSAGE_TIMEOUT_FLAGS(SMTO_ABORTIFHUNG.0 | SMTO_BLOCK.0);

    let success = unsafe {
        SendMessageTimeoutW(
            hwnd,
            msg,
            WPARAM(wparam),
            LPARAM(lparam),
            flags,
            timeout_ms,
            Some(&mut result),
        )
    };

    if success.0 == 0 {
        Err(windows::core::Error::from_win32())
    } else {
        Ok(result)
    }
}

/// 获取窗口类名
pub fn get_class_name(hwnd: HWND) -> Result<String> {
    unsafe {
        // 创建缓冲区存储类名
        let mut buffer = [0u16; 256];
        // GetClassNameW 返回复制的字符数（不包括结尾的null）
        let length = GetClassNameW(hwnd, &mut buffer);
        // if length == 0 {
        //     return Err(windows::core::Error::from_win32());
        // }

        // 转换为String，只使用实际长度的部分
        Ok(String::from_utf16_lossy(&buffer[..length as usize]))
    }
}
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
/// 获取当前鼠标坐标
pub fn get_cursor_position() -> Result<(i32, i32)> {
    unsafe {
        let mut point = POINT::default();
        GetCursorPos(&mut point).ok();
        Ok((point.x, point.y))
    }
}

//// 获取相对于指定窗口的鼠标坐标
// fn get_cursor_position_relative_to_window(hwnd: HWND) -> Result<(i32, i32), windows::core::Error> {
//     unsafe {
//         let mut point = POINT::default();
//         GetCursorPos(&mut point).ok()?;
//         ScreenToClient(hwnd, &mut point).ok()?;
//         Ok((point.x, point.y))
//     }
// }

// pub enum CwpFLAGS {
//     CWP_ALL,
//     // CWP_ALL = CWP_ALL,
//     // CWP_SKIPINVISIBLE = CWP_SKIPINVISIBLE,
//     // CWP_SKIPDISABLED = CWP_SKIPDISABLED,
//     // CWP_SKIPTRANSPAREMagic = CWP_SKIPTRANSPAREMagic,
// }
// pub fn child_window_from_point_ex(hwnd: HWND, pt: POINT, flags: CwpFLAGS) -> Option<HWND> {
//     let ret = unsafe { ChildWindowFromPointEx(hwnd, pt, flags.) };
//     return Some(ret);
// }
