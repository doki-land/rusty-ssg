use serde::{Deserialize};
use std::borrow::Cow;

mod error;
mod login;

pub use crate::{
    error::WechatError,
    login::{access_token::WechatAccessToken, phone_number::WechatPhoneNumber, session_key::WechatSession},
};
#[derive(Clone, Debug, Deserialize)]
pub struct MiniProgram {
    /// 小程序的 `app_id`
    pub app_id: Cow<'static, str>,
    /// 小程序的 `app_secret`
    pub secret: Cow<'static, str>,
}
