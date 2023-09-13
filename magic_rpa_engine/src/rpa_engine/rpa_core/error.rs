use core::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct Error {
    code: i32,
    message: String,
}
impl Error {
    pub fn new(code: i32, message: &str) -> Error {
        Error {
            code,
            message: String::from(message),
        }
    }

    // pub fn last_os_error() -> Error {
    //     let error = unsafe { GetLastError() };
    //     // let code: i32 = if (error.0 as i32) < 0 {
    //     //     error.0 as _
    //     // } else {
    //     //     ((error.0 & 0x0000FFFF) | 0x80070000) as _
    //     // };

    //     // HRESULT(code).into()
    //     if let Err(e) = error {
    //         e.into()
    //     } else {
    //         HRESULT(0).into()
    //     }
    // }

    pub fn code(&self) -> i32 {
        self.code
    }

    // pub fn result(&self) -> Option<HRESULT> {
    //     if self.code < 0 {
    //         Some(HRESULT(self.code))
    //     } else {
    //         None
    //     }
    // }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {}

impl From<windows::core::Error> for Error {
    fn from(e: windows::core::Error) -> Self {
        Self {
            code: e.code().0,
            message: e.message().to_string(),
        }
    }
}
impl From<uiautomation::Error> for Error {
    fn from(e: uiautomation::Error) -> Self {
        Self {
            code: e.code(),
            message: e.message().to_string(),
        }
    }
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Error { code: 0, message }
    }
}

impl From<&str> for Error {
    fn from(message: &str) -> Self {
        Error {
            code: 0,
            message: String::from(message),
        }
    }
}

pub type Result<T> = core::result::Result<T, uiautomation::Error>;
