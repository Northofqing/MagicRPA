// use windows::{
//     Win32::Foundation::*,
//     Win32::UI::WindowsAndMessaging::*,
//     Win32::UI::Accessibility::*,
//     Win32::System::Ole::*,
//     core::*,
// };

// /// 获取指定点的可访问性对象信息
// fn get_accessible_info_at_point(x: i32, y: i32) -> Result<AccessibleInfo> {
//     unsafe {
//         let mut accessible = None;
//         let mut child_id = 0;
//         let point = POIMagic { x, y };

//         // 获取指定位置的可访问性对象
//         AccessibleObjectFromPoint(
//             point,
//             &mut accessible,
//             &mut child_id
//         )?;

//         // 如果找到了可访问性对象
//         if let Some(acc) = accessible {
//             let variant = VARIAMagic::default();
            
//             // 获取各种属性
//             let name = match acc.accName(variant.clone()) {
//                 Ok(name) => name.to_string(),
//                 Err(_) => String::new(),
//             };

//             let role = match acc.accRole(variant.clone()) {
//                 Ok(role) => role.to_string(),
//                 Err(_) => String::new(),
//             };

//             let value = match acc.accValue(variant.clone()) {
//                 Ok(value) => value.to_string(),
//                 Err(_) => String::new(),
//             };

//             let description = match acc.accDescription(variant.clone()) {
//                 Ok(desc) => desc.to_string(),
//                 Err(_) => String::new(),
//             };

//             let help = match acc.accHelp(variant.clone()) {
//                 Ok(help) => help.to_string(),
//                 Err(_) => String::new(),
//             };
            
//             // 获取状态
//             let state = match acc.accState(variant) {
//                 Ok(state) => state.Anonymous.lVal,
//                 Err(_) => 0,
//             };

//             // 创建返回结构
//             Ok(AccessibleInfo {
//                 name,
//                 role,
//                 value,
//                 description,
//                 help,
//                 state,
//                 child_id,
//             })
//         } else {
//             Err(Error::from_win32())
//         }
//     }
// }

// #[derive(Debug)]
// struct AccessibleInfo {
//     name: String,
//     role: String,
//     value: String,
//     description: String,
//     help: String,
//     state: i32,
//     child_id: i32,
// }

// impl AccessibleInfo {
//     fn get_state_description(&self) -> Vec<String> {
//         let mut states = Vec::new();
        
//         // 使用正确的可访问性状态常量
//         let state_checks = [
//             (ACCESSIBLE_STATE_SYSTEM_FLAGS(1), "Unavailable"),
//             (ACCESSIBLE_STATE_SYSTEM_FLAGS(2), "Selected"),
//             (ACCESSIBLE_STATE_SYSTEM_FLAGS(4), "Focused"),
//             (ACCESSIBLE_STATE_SYSTEM_FLAGS(8), "Pressed"),
//             (ACCESSIBLE_STATE_SYSTEM_FLAGS(16), "Checked"),
//             (ACCESSIBLE_STATE_SYSTEM_FLAGS(32), "Mixed"),
//             (ACCESSIBLE_STATE_SYSTEM_FLAGS(64), "ReadOnly"),
//             (ACCESSIBLE_STATE_SYSTEM_FLAGS(128), "HotTracked"),
//             (ACCESSIBLE_STATE_SYSTEM_FLAGS(256), "Default"),
//             (ACCESSIBLE_STATE_SYSTEM_FLAGS(512), "Expanded"),
//             (ACCESSIBLE_STATE_SYSTEM_FLAGS(1024), "Collapsed"),
//             (ACCESSIBLE_STATE_SYSTEM_FLAGS(2048), "Busy"),
//             (ACCESSIBLE_STATE_SYSTEM_FLAGS(32768), "Invisible"),
//             (ACCESSIBLE_STATE_SYSTEM_FLAGS(65536), "Offscreen"),
//             (ACCESSIBLE_STATE_SYSTEM_FLAGS(131072), "Sizeable"),
//             (ACCESSIBLE_STATE_SYSTEM_FLAGS(262144), "Moveable"),
//         ];

//         for (flag, description) in state_checks {
//             if self.state & flag.0 != 0 {
//                 states.push(description.to_string());
//             }
//         }
        
//         states
//     }
// }

// fn main() -> Result<()> {
//     // 获取当前鼠标位置
//     let mut point = POIMagic::default();
//     unsafe { GetCursorPos(&mut point).ok()? };

//     println!("Getting accessible info at position ({}, {})", point.x, point.y);

//     // 获取该位置的可访问性信息
//     match get_accessible_info_at_point(point.x, point.y) {
//         Ok(info) => {
//             println!("\nAccessible Object Information:");
//             println!("Name: {}", info.name);
//             println!("Role: {}", info.role);
//             println!("Value: {}", info.value);
//             println!("Description: {}", info.description);
//             println!("Help: {}", info.help);
//             println!("Child ID: {}", info.child_id);
//             println!("States:");
//             for state in info.get_state_description() {
//                 println!("  - {}", state);
//             }
//         }
//         Err(e) => println!("Error getting accessible info: {:?}", e),
//     }

//     Ok(())
// }