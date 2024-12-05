use serde::{Deserialize, Serialize};
use std::borrow::Cow;

mod login;

pub use crate::login::access_token::WechatAccessToken;

#[derive(Clone, Debug, Deserialize)]
pub struct MiniProgram {
    /// 小程序的 `app_id`
    pub app_id: Cow<'static, str>,
    /// 小程序的 `app_secret`
    pub secret: Cow<'static, str>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
// #[cfg_attr(feature = "poem-openapi", derive(poem_openapi::Object))]
pub struct WechatSessionResponse {
    #[serde(rename = "errcode")]
    pub error_code: Option<i32>,
    #[serde(default, rename = "errmsg")]
    pub error_message: String,
    #[serde(flatten)]
    pub session_info: WechatSession,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WechatSession {
    #[serde(default, rename = "openid")]
    pub open_id: String,
    #[serde(default, rename = "unionid")]
    /// <https://developers.weixin.qq.com/miniprogram/dev/framework/open-ability/union-id.html>
    pub union_id: String,
    #[serde(default, rename = "session_key")]
    pub session_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WechatPhoneResponse {
    #[serde(rename = "errcode")]
    pub error_code: Option<i32>,
    #[serde(default, rename = "errmsg")]
    pub error_message: String,
    pub phone_info: WechatPhone,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WechatPhone {
    /// 用户绑定的手机号（国外手机号会有区号）
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
    /// 没有区号的手机号
    #[serde(rename = "purePhoneNumber")]
    pub phone_number_pure: String,
    /// 区号
    #[serde(rename = "countryCode")]
    pub country_code: String,
    /// 数据水印
    pub watermark: WechatPhoneWatermark,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WechatPhoneWatermark {
    /// 用户获取手机号操作的时间戳
    timestamp: f64,
    /// 小程序 app_id
    #[serde(rename = "appid")]
    app_id: String,
}

impl WechatSessionResponse {
    pub fn as_result(&self) -> Result<WechatSession, (i32, &str)> {
        match self.error_code {
            Some(0) | None => Ok(self.session_info.clone()),
            Some(-1) => Err((-1, "微信服务器系统繁忙, 请稍后再试")),
            Some(40029) => Err((40029, "无效的 login_code")),
            Some(45011) => Err((45011, "频率限制, 每个用户每分钟最多 100 次尝试")),
            Some(40163) => Err((40163, "该登录 code 已被使用")),
            Some(40226) => Err((40226, "高风险用户, 已被微信禁止登录")),
            Some(i) => Err((i, &self.error_message)),
        }
    }
}

impl WechatPhoneResponse {
    pub fn as_result(&self) -> Result<WechatPhone, (i32, &str)> {
        match self.error_code {
            Some(0) | None => Ok(self.phone_info.clone()),
            Some(-1) => Err((-1, "微信服务器系统繁忙, 请稍后再试")),
            Some(40029) => Err((40029, "无效的 phone_code")),
            Some(45011) => Err((45011, "频率限制, 每个用户每分钟最多 100 次尝试")),
            Some(40163) => Err((40163, "该登录 code 已被使用")),
            Some(40226) => Err((40226, "高风险用户, 已被微信禁止登录")),
            Some(i) => Err((i, &self.error_message)),
        }
    }
}
