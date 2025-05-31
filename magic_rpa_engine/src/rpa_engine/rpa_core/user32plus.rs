use windows::Win32::Foundation::{HWND, LPARAM, BOOL};
use windows::Win32::UI::WindowsAndMessaging::EnumChildWindows;
use std::error::Error;

#[derive(Debug)]
pub enum WindowEnumError {
    EnumFailed(String),
    InvalidWindow,
}

pub fn query_all_child_windows<F>(
    hwnd_parent: HWND,
    predicate: F
) -> Result<Vec<HWND>, WindowEnumError>
where
    F: Fn(HWND) -> bool
{
    if hwnd_parent == HWND::default() {
        return Err(WindowEnumError::InvalidWindow);
    }

    struct Context<F> {
        predicate: F,
        results: Vec<HWND>,
    }

    unsafe extern "system" fn callback<F: Fn(HWND) -> bool>(
        hwnd: HWND,
        lparam: LPARAM
    ) -> BOOL {
        let context = &mut *(lparam.0 as *mut Context<F>);
        if (context.predicate)(hwnd) {
            context.results.push(hwnd);
        }
        BOOL::from(true)
    }

    let mut context = Context {
        predicate,
        results: Vec::new(),
    };

    let success = unsafe {
        EnumChildWindows(
            hwnd_parent,
            Some(callback::<F>),
            LPARAM(&mut context as *mut _ as isize),
        ).as_bool()
    };

    if !success {
        return Err(WindowEnumError::EnumFailed(
            "EnumChildWindows failed".to_string()
        ));
    }

    Ok(context.results)
}