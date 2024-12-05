use super::*;
use serde::{Deserialize, Deserializer};
use std::time::SystemTime;

#[derive(Clone, Debug)]
pub struct WechatAccessToken {
    /// 获取到的凭证
    pub access_token: String,
    /// 凭证过期时间
    pub expiration_time: SystemTime,
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
            .json::<WechatAccessToken>()
            .await?;
        Ok(response)
    }
}

impl WechatAccessToken {
    pub async fn update(&mut self, app: MiniProgram) -> Result<(), reqwest::Error> {
        if self.needs_update() {
            *self = app.get_access_token().await?;
        }
        Ok(())
    }
    pub fn needs_update(&self) -> bool {
        self.expiration_time < SystemTime::now()
    }
    pub async fn force_update(&mut self, app: MiniProgram) -> Result<(), reqwest::Error> {
        *self = app.get_access_token().await?;
        Ok(())
    }
}

#[derive(Default, Deserialize)]
struct AccessTokenVisitor {
    access_token: String,
    expires_in: u64,
}

impl<'de> Deserialize<'de> for WechatAccessToken {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let response = AccessTokenVisitor::deserialize(deserializer)?;
        Ok(WechatAccessToken {
            access_token: response.access_token,
            expiration_time: SystemTime::now() + std::time::Duration::from_secs(response.expires_in),
        })
    }
}
