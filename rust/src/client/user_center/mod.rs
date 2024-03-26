use crate::client::types::ClientResult;
use crate::client::Client;
use serde_json::json;

use self::dataobject::{Login, Profile};

pub mod dataobject;
#[cfg(test)]
mod tests;

impl Client {
    pub async fn register(
        &self,
        username: &str,
        password: &str,
        email: &str,
        invitecode: &str,
    ) -> ClientResult<i64> {
        let rsp = self
            .bbs_post(
                "/apicloud/",
                json!({
                    "mod": "register",
                }),
                json!({
                    "username": username,
                    "password": password,
                    "email": email,
                    "invitecode": invitecode,
                }),
            )
            .await?;
        let data = self.json_errcode_field(rsp, "uid").await?;
        Ok(data)
    }

    pub async fn profile(&self, uid: i64) -> ClientResult<Profile> {
        let rsp = self
            .bbs_post(
                "/apicloud/",
                json!({
                    "mod": "profile",
                }),
                json!({
                    "uid": uid,
                }),
            )
            .await?;
        let data = self.json_errcode_field(rsp, "profile").await?;
        Ok(data)
    }

    pub async fn login(&self, username: &str, password: &str) -> ClientResult<Login> {
        let rsp = self
            .bbs_post(
                "/apicloud/",
                json!({
                    "mod": "login",
                }),
                json!({
                    "key": "apicloud",
                    "username": username,
                    "password": password,
                    "sha": self.sha(),
                }),
            )
            .await?;
        let data = self.json_errcode_else(rsp).await?;
        Ok(data)
    }

    fn sha(&self) -> String {
        // sha生成方式:
        // sha_1 = 取key的2-3位,即pi
        // sha_2 = sha_1 + 4位数年份
        // sha_3 = md5(sha_2) + 2位数月份
        // sha_4 = md5(sha_3) + 2位数天数
        // sha_5 = 取key的1-3位,即api
        // sha_6 = sha_4 + sha_5
        // sha = md5(sha_6)
        let now = chrono::Local::now();
        let sha_1 = "pi";
        let sha_2 = format!("{}{}", sha_1, now.format("%Y").to_string());
        let sha_3 = format!(
            "{}{}",
            hex::encode(md5::compute(sha_2.as_bytes()).0),
            now.format("%m").to_string()
        );
        let sha_4 = format!(
            "{}{}",
            hex::encode(md5::compute(sha_3.as_bytes()).0),
            now.format("%d").to_string()
        );
        let sha_5 = "api";
        let sha_6 = format!("{}{}", sha_4, sha_5);
        let sha = hex::encode(md5::compute(sha_6.as_bytes()).0);
        sha
    }
}
