use super::*;
use serde::{Deserialize, Deserializer, de::Error};

#[derive(Clone, Debug)]
pub struct WechatPhoneNumber {
    /// 用户绑定的手机号（国外手机号会有区号）
    pub phone_number: String,
    /// 没有区号的手机号
    pub phone_number_pure: String,
    /// 区号
    pub country_code: String,
    /// 用户获取手机号操作的时间戳
    pub timestamp: f64,
    /// 小程序 app_id
    pub app_id: String,
}

impl MiniProgram {
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/user-login/code2Session.html>
    pub async fn get_phone_number(&self, code: &str, access_token: &str) -> Result<WechatPhoneNumber, reqwest::Error> {
        let mut params = HashMap::new();
        params.insert("access_token", access_token);
        let mut body = Map::with_capacity(1);
        body.insert("code".to_string(), Value::String(code.to_string()));
        let client = reqwest::Client::new();
        let response = client
            .post("https://api.weixin.qq.com/wxa/business/getuserphonenumber")
            .query(&params)
            .header(CONTENT_TYPE, "application/json")
            .body(Value::Object(body).to_string())
            .send()
            .await?
            .json::<WechatPhoneVisitor>()
            .await?;
        match response.errcode {
            Some(0) | None => Ok(WechatPhoneNumber {
                app_id: response.phone_info.watermark.appid,
                phone_number: response.phone_info.phoneNumber,
                country_code: response.phone_info.countryCode,
                phone_number_pure: response.phone_info.purePhoneNumber,
                timestamp: response.phone_info.watermark.timestamp,
            }),
            Some(-1) => {
                Err(Error::custom("[WechatError=-1] The wechat server system is busy, please try again later.".to_string()))
            }
            Some(40029) => Err(Error::custom("[WechatError=40029] Invalid `phone_code`.".to_string())),
            Some(40163) => Err(Error::custom("[WechatError=40163] The login code has been used.".to_string())),
            Some(40226) => Err(Error::custom("[WechatError=40226] High-risk user, has been banned by wechat.".to_string())),
            Some(45011) => {
                Err(Error::custom("[WechatError=45011] Rate limit, up to 100 attempts per minute per user.".to_string()))
            }
            Some(i) => Err(Error::custom(format!("[WechatError={}] {}", i, response.errmsg))),
        }
    }
}

#[derive(Deserialize)]
struct WechatPhoneVisitor {
    errcode: Option<i32>,
    errmsg: String,
    phone_info: PhoneInfo,
}

#[derive(Deserialize)]
struct PhoneInfo {
    phoneNumber: String,
    purePhoneNumber: String,
    countryCode: String,
    watermark: PhoneWatermark,
}

#[derive(Deserialize)]
struct PhoneWatermark {
    timestamp: f64,
    appid: String,
}
