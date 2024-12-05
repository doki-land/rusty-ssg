use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
};

pub struct WechatError {
    pub code: i32,
    pub message: Cow<'static, str>,
}

impl WechatError {
    pub fn builtin<T>(code: i32, message: &'static str) -> Result<T, Self> {
        Err(WechatError { code, message: Cow::Borrowed(message) })
    }
    pub fn unknown<T, S: ToString>(code: i32, message: S) -> Result<T, Self> {
        Err(WechatError { code, message: Cow::Owned(message.to_string()) })
    }
}

impl From<reqwest::Error> for WechatError {
    fn from(error: reqwest::Error) -> Self {
        WechatError { code: -200, message: Cow::Owned(error.to_string()) }
    }
}

impl Display for WechatError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[WechatError={}] {}", self.code, self.message)
    }
}
