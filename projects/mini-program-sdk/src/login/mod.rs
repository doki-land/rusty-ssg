use std::collections::HashMap;
use reqwest::header::CONTENT_TYPE;
use serde_json::{Map, Value};
use crate::{MiniProgram, WechatSessionResponse};

pub mod access_token;
pub mod phone_number;

impl MiniProgram {
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/user-login/code2Session.html>
    pub async fn code2session(&self, js_code: &str) -> Result<WechatSessionResponse, reqwest::Error> {
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
            .json::<WechatSessionResponse>()
            .await?;
        Ok(response)
    }
    /// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/user-login/code2Session.html>
    pub async fn get_phone_number(&self, code: &str, access_token: &str) -> Result<WechatSessionResponse, reqwest::Error> {
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
            .json::<WechatSessionResponse>()
            .await?;
        Ok(response)
    }
}
