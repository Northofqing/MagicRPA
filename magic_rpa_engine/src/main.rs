//#![windows_subsystem = "windows"]
#![allow(
    dead_code,
    unused_variables,
    unused_attributes,
    unused_imports,
    non_snake_case,
    unused_assignments,
    unused_must_use,
    unused_mut,
    unconditional_recursion,
    unused_unsafe,
    unreachable_patterns
)]
#![warn(clippy::all)] // 启用所有检查
mod rpa_engine;
mod rpa_log;
mod rpa_proxy;
mod rpa_windows;

use crate::rpa_proxy::spy::inspector::InspectorInner;

use crate::rpa_engine::rpa_core::error;
use crate::rpa_engine::rpa_core::point::MagicPoint;
use crate::rpa_log::log_config::*;
use globle_hook::handlers::InputEventHandler;
use globle_hook::monitor::InputMonitor;
use log::{debug, error, info, trace, warn};
use rpa_engine::rpa_automation::rpa_win32::win32automation::Win32Automation;
use rpa_proxy::spy::ui_spy::UISpy;
use rpa_windows::highlight_windows::{self, *};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use autoit_wrapper::{autoit_error::AutoItError, AutoIt};

use std::error as other_error;
fn main() {
    // 初始化 AutoIt
    AutoIt::init();
    // AutoIt::mouse_click("left", 10, 10,1, 100);
    // // 创建日志管理器
    // let log_manager = Arc::new(LogManager::new());

    // // 启动配置监控
    // LogManager::start_config_monitor(log_manager.clone());

    let window = TransparentWindow::new().unwrap();
    // start
    let start = Instant::now();
    loop {
        let point = MagicPoint::get_cursor_pos();

        let ins = InspectorInner::new();
        let trace_data = ins.inspect_element_point_spy(point);
        // Self::inspect_element_point_single(point);
        // let rect = ins.inspect_element_point_spy(point.clone());
        // let _ = ins.inspect_element_point_single(point);
        let rect = trace_data.unwrap().bounding;
        window.set_position(rect.x, rect.y);
        window.set_size(rect.width, rect.height);
        // // ... 其他操作

        // info!("element bounding: {:?}", rect);
        // ending time
        // let duration = start.elapsed();

        // info!("Elapsed time: {:?}", duration);
        // info!("_____分割线______");
    }
    //window.close();
    // let mut monitor = InputMonitor::new().unwrap();

    // // 添加统一的事件处理器，可以控制是否阻止移动事件
    // monitor.add_handler(InputEventHandler::new(false)); // false表示不阻止移动事件穿透

    // type SafeWindow = Arc<Mutex<TransparentWindow>>;

    // // 创建实例
    // let window: SafeWindow = TransparentWindow::new()
    //     .map(|w| Arc::new(Mutex::new(w)))
    //     .expect("Failed to create window");
    // let window_for_chinge = Arc::clone(&window);
    // // 运行监控器
    // monitor.run(move || {
    //     // start
    //     let start = Instant::now();
    //     let point = MagicPoint::get_cursor_pos();
    //     let ins = InspectorInner::new();
    //     //Self::inspect_element_point_single(point);
    //     let rect = ins.inspect_element_point_spy(point.clone());
    //     let _ = ins.inspect_element_point_single(point);
    //     let _ = window_for_chinge
    //         .lock()
    //         .map(|window| {
    //             window.set_position(rect.x, rect.y);
    //             window.set_size(rect.width, rect.height);
    //             // ... 其他操作
    //         })
    //         .map_err(|e| error!("Failed to lock window: {}", e));

    //     info!("element bounding: {:?}", rect);
    //     // ending time
    //     let duration = start.elapsed();

    //     info!("Elapsed time: {:?}", duration);
    //     info!("_____分割线______");
    // });
    // let window_for_close = Arc::clone(&window);
    // let _ = window_for_close
    //     .lock()
    //     .map(|window| {
    //         window.close();
    //     })
    //     .map_err(|e| error!("Failed to lock window: {}", e));
}
