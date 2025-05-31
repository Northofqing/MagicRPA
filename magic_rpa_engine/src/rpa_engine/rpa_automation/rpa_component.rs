use crate::rpa_engine::rpa_core::{error::Result, pinvoke, point::MagicPoint, rect::MagicRect};
use windows::Win32::Foundation::HWND;

// pub trait RPAWin32Component {
//     fn handle(&self) -> HWND;
//     fn class_name(&self) -> String;
//     fn title(&self) -> String;
//     fn is_visible(&self) -> bool;
//     fn is_enabled(&self) -> bool;
//     fn bounding(&self) -> MagicRect;
//     fn process_id(&mut self) -> u32;
//     fn clone_box(&self) -> Box<dyn RPAWin32Component>;
// }