use super::*;
use crate::login::phone_number::WechatPhoneNumber;
use serde::{Deserialize, Serialize, de::Error};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WechatSession {
    pub open_id: String,
    /// <https://developers.weixin.qq.com/miniprogram/dev/framework/open-ability/union-id.html>
    pub union_id: String,
    pub session_key: String,
}

impl MiniProgram {
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/user-login/code2Session.html>
    pub async fn code2session(&self, js_code: &str) -> Result<WechatSession, reqwest::Error> {
        let mut params = HashMap::new();
        params.insert("appid", self.app_id.as_ref());
        params.insert("secret", self.secret.as_ref());
        params.insert("js_code", js_code);
        params.insert("grant_type", "authorization_code");
        let client = reqwest::Client::new();
        let response = client
            .get("https://api.weixin.qq.com/sns/jscode2session")
            .query(&params)
            .header(CONTENT_TYPE, "application/json")
            // .header(USER_AGENT, "your-app-name")
            .send()
            .await?
            .json::<SessionVisitor>()
            .await?;
        match response.errcode {
            Some(0) | None => {
                Ok(WechatSession { open_id: response.openid, union_id: response.unionid, session_key: response.session_key })
            }
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
struct SessionVisitor {
    errcode: Option<i32>,
    errmsg: String,
    openid: String,
    unionid: String,
    session_key: String,
}
