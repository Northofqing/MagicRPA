use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::Graphics::Gdi::*,
    Win32::System::LibraryLoader::*,
    Win32::UI::WindowsAndMessaging::*,
};
use std::sync::mpsc::{channel, Sender};
use std::thread;

// 窗口命令枚举
#[derive(Clone)]
enum WindowCommand {
    Move(i32, i32),
    Resize(i32, i32),
    Close,
}
#[derive(Clone)]
pub struct TransparentWindow {
    command_sender: Sender<WindowCommand>,
}

impl TransparentWindow {
    pub fn new() -> windows::core::Result<Self> {
        let (tx, rx) = channel();
        let command_sender = tx.clone();

        // 在新线程中创建和运行窗口
        thread::spawn(move || -> windows::core::Result<()> {
            unsafe {
                let instance = GetModuleHandleW(None)?;
                let window_class = w!("transparent_window");
                
                let wc = WNDCLASSEXW {
                    cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
                    style: CS_HREDRAW | CS_VREDRAW,
                    lpfnWndProc: Some(wnd_proc),
                    hInstance: instance.into(),
                    lpszClassName: PCWSTR(window_class.as_ptr()),
                    ..Default::default()
                };
                
                RegisterClassExW(&wc);

                let hwnd = CreateWindowExW(
                    WINDOW_EX_STYLE(WS_EX_LAYERED.0 | WS_EX_TRANSPARENT.0 | WS_EX_TOOLWINDOW.0),
                    window_class,
                    w!("透明窗口"),
                    WINDOW_STYLE(WS_POPUP.0 | WS_VISIBLE.0),
                    100, 100,
                    400, 300,
                    None,
                    None,
                    instance,
                    None,
                ).unwrap();

                if hwnd==HWND::default() {
                    return Err(Error::from_win32());
                }

                SetLayeredWindowAttributes(
                    hwnd,
                    COLORREF(0),
                    100,
                    LAYERED_WINDOW_ATTRIBUTES_FLAGS(LWA_ALPHA.0),
                );

                SetWindowPos(
                    hwnd,
                    HWND_TOPMOST,
                    0, 0, 0, 0,
                    SET_WINDOW_POS_FLAGS(SWP_NOMOVE.0 | SWP_NOSIZE.0 | SWP_NOACTIVATE.0)
                );

                // 创建一个定时器来处理命令
                let timer_id = 1;
                SetTimer(hwnd, timer_id, 10, None); // 10ms 检查一次命令

                // 将接收器存储在窗口的用户数据中
                let rx_ptr = Box::into_raw(Box::new(rx));
                SetWindowLongPtrW(hwnd, GWLP_USERDATA, rx_ptr as _);

                // 消息循环
                let mut message = MSG::default();
                while GetMessageW(&mut message, HWND::default(), 0, 0).into() {
                    TranslateMessage(&message);
                    DispatchMessageW(&message);
                }

                // 清理接收器
                drop(Box::from_raw(rx_ptr));
                Ok(())
            }
        });

        Ok(Self { command_sender: tx })
    }

    pub fn set_position(&self, x: i32, y: i32) {
        let _ = self.command_sender.send(WindowCommand::Move(x, y));
    }

    pub fn set_size(&self, width: i32, height: i32) {
        let _ = self.command_sender.send(WindowCommand::Resize(width, height));
    }

    pub fn close(&self) {
        let _ = self.command_sender.send(WindowCommand::Close);
    }
}

extern "system" fn wnd_proc(hwnd: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_PAINT => {
                let mut ps = PAINTSTRUCT::default();
                let hdc = BeginPaint(hwnd, &mut ps);
                
                let red_pen = CreatePen(PEN_STYLE(PS_SOLID.0), 3, COLORREF(0x0000FF));
                let old_pen = SelectObject(hdc, red_pen);
                
                let mut rect = RECT::default();
                GetClientRect(hwnd, &mut rect);
                
                Rectangle(
                    hdc,
                    0,
                    0,
                    rect.right,
                    rect.bottom,
                );
                
                SelectObject(hdc, old_pen);
                DeleteObject(red_pen);
                EndPaint(hwnd, &ps);
                LRESULT(0)
            }

            WM_TIMER => {
                // 获取接收器并检查命令
                let rx_ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut std::sync::mpsc::Receiver<WindowCommand>;
                if !rx_ptr.is_null() {
                    let rx = &*rx_ptr;
                    while let Ok(command) = rx.try_recv() {
                        match command {
                            WindowCommand::Move(x, y) => {
                                SetWindowPos(
                                    hwnd,
                                    HWND_TOPMOST,
                                    x, y,
                                    0, 0,
                                    SET_WINDOW_POS_FLAGS(SWP_NOSIZE.0 | SWP_NOACTIVATE.0)
                                );
                            },
                            WindowCommand::Resize(width, height) => {
                                SetWindowPos(
                                    hwnd,
                                    HWND_TOPMOST,
                                    0, 0,
                                    width, height,
                                    SET_WINDOW_POS_FLAGS(SWP_NOMOVE.0 | SWP_NOACTIVATE.0)
                                );
                            },
                            WindowCommand::Close => {
                                DestroyWindow(hwnd);
                            }
                        }
                    }
                }
                LRESULT(0)
            }

            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }
            
            _ => DefWindowProcW(hwnd, message, wparam, lparam),
        }
    }
}