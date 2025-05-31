use windows::Win32::Foundation::HWND;

use super::{rpa_spy::ISpy, spy_target::SpyTarget};

pub struct MatchedSpy {
    pub hwnd: HWND,
    pub spy: Box<dyn ISpy>,
    pub target: SpyTarget,
}