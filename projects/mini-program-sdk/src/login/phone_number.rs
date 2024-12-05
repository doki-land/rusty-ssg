use super::*;
use serde::{Deserialize, Deserializer, Serialize, de::Error};

#[derive(Clone, Debug)]
pub struct WechatPhoneNumber {
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

#[derive(Clone, Debug, Deserialize)]
pub struct WechatPhoneWatermark {
    /// 用户获取手机号操作的时间戳
    timestamp: f64,
    /// 小程序 app_id
    #[serde(rename = "appid")]
    app_id: String,
}

#[derive(Deserialize)]
pub struct WechatPhoneVisitor {
    #[serde(rename = "errcode")]
    pub error_code: Option<i32>,
    #[serde(default, rename = "errmsg")]
    pub error_message: String,
    pub phone_info: WechatPhoneNumber,
}

impl MiniProgram {
    /// <https://developers.weixin.qq.com/miniprogram/dev/platform-capabilities/miniapp/openapi/getaccesstoken.html>
    pub async fn get_access_token(&self) -> Result<WechatAccessToken, reqwest::Error> {
        let mut params = HashMap::new();
        params.insert("appid", self.app_id.as_ref());
        params.insert("secret", self.secret.as_ref());
        params.insert("grant_type", "client_credential");
        let client = reqwest::Client::new();
        let response = client
            .get("https://api.weixin.qq.com/cgi-bin/token")
            .query(&params)
            .header(CONTENT_TYPE, "application/json")
            // .header(USER_AGENT, "your-app-name")
            .send()
            .await?
            .json::<WechatPhoneNumber>()
            .await?;
        Ok(response)
    }
}

impl<'de> Deserialize<'de> for WechatPhoneNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let response = WechatPhoneVisitor::deserialize(deserializer)?;
        match response.error_code {
            Some(0) | None => Ok(response.phone_info),
            Some(-1) => {
                Err(Error::custom("[WechatError=-1] The wechat server system is busy, please try again later.".to_string()))
            }
            Some(40029) => Err(Error::custom("[WechatError=40029] Invalid `phone_code`.".to_string())),
            Some(40163) => Err(Error::custom("[WechatError=40163] The login code has been used.".to_string())),
            Some(40226) => Err(Error::custom("[WechatError=40226] High-risk user, has been banned by wechat.".to_string())),
            Some(45011) => {
                Err(Error::custom("[WechatError=45011] Rate limit, up to 100 attempts per minute per user.".to_string()))
            }
            Some(i) => Err(Error::custom(format!("[WechatError={}] {}", i, response.error_message))),
        }
    }
}
