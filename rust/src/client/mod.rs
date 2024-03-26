pub mod community;
pub mod fuli;
#[cfg(test)]
mod tests;
pub mod types;
pub mod user_center;
pub mod utils;

use self::types::ClientResult;
use crate::client::types::ClientError;
use crate::domain::cookie_jar::CookieJar;
use reqwest::header::COOKIE;
use serde::Serialize;
use std::fmt::Debug;
use url::Url;
use utils::{from_json_str, from_json_value};

const BBS_HOST: &'static str = "https://windowsupdatebbs.fuliba.app";
const HOST: &'static str = "https://windowsupdate.fuliba.app";

#[derive(Debug)]
pub struct Client {
    cookie_jar: CookieJar,
    client: reqwest::Client,
}

impl Client {
    pub fn new(cookie_jar: CookieJar) -> Self {
        let client = reqwest::Client::new();
        Self { cookie_jar, client }
    }

    pub async fn request<T: Serialize + Debug>(
        &self,
        method: reqwest::Method,
        host: &str,
        path: &str,
        query: impl Into<Option<T>>,
        form: impl Into<Option<T>>,
    ) -> ClientResult<reqwest::Response> {
        let url = Url::parse(format!("{}{}", host, path).as_str())?;
        let query = query.into();
        let form = form.into();
        let cookies = self.cookie_jar.cookies(&url).await;
        #[cfg(test)]
        {
            debug!("==================================================");
            debug!("request method: {}", method);
            debug!("request url: {}", url);
            debug!("request query: {:?}", query);
            debug!("request cookies: {:?}", cookies);
            debug!("request form: {:?}", form);
        }
        let req = self.client.request(method, url.as_str());
        let req = if let Some(cookies) = cookies {
            req.header(COOKIE, cookies)
        } else {
            req
        };
        let req = match query {
            Some(query) => req.query(&query),
            None => req,
        };
        let req = match form {
            Some(form) => req.form(&form),
            None => req,
        };
        Ok(req.send().await?)
    }

    pub async fn api_request<T: Serialize + Debug>(
        &self,
        method: reqwest::Method,
        path: &str,
        query: impl Into<Option<T>>,
        form: impl Into<Option<T>>,
    ) -> ClientResult<reqwest::Response> {
        self.request(method, HOST, path, query, form).await
    }

    pub async fn api_get<T: Serialize + Debug>(
        &self,
        path: &str,
        query: impl Into<Option<T>>,
    ) -> ClientResult<reqwest::Response> {
        self.api_request(reqwest::Method::GET, path, query, None)
            .await
    }

    pub async fn bbs_request<T: Serialize + Debug>(
        &self,
        method: reqwest::Method,
        path: &str,
        query: impl Into<Option<T>>,
        form: impl Into<Option<T>>,
    ) -> ClientResult<reqwest::Response> {
        self.request(method, BBS_HOST, path, query, form).await
    }

    pub async fn bbs_get<T: Serialize + Debug>(
        &self,
        path: &str,
        query: impl Into<Option<T>>,
    ) -> ClientResult<reqwest::Response> {
        self.bbs_request(reqwest::Method::GET, path, query, None)
            .await
    }

    pub async fn bbs_post<T: Serialize + Debug>(
        &self,
        path: &str,
        query: impl Into<Option<T>>,
        form: impl Into<Option<T>>,
    ) -> ClientResult<reqwest::Response> {
        self.bbs_request(reqwest::Method::POST, path, query, form)
            .await
    }

    pub async fn response_text(&self, response: reqwest::Response) -> ClientResult<String> {
        let headers = response.headers();
        self.cookie_jar
            .set_cookies(
                headers
                    .iter()
                    .filter(|(name, _value)| name.as_str().to_lowercase() == "set-cookie")
                    .map(|(_name, value)| value)
                    .map(Clone::clone)
                    .collect(),
                response.url(),
            )
            .await;
        #[cfg(test)]
        {
            debug!("--------------------------------------------------");
            debug!("response status: {:?}", response.status());
            debug!("response headers:");
            for (name, value) in headers {
                debug!("   {} : {:?}", name, value);
            }
        }
        let text = response.text().await?;
        #[cfg(test)]
        {
            debug!("response body : {}", text.as_str());
            debug!("==================================================")
        }
        Ok(text)
    }

    pub async fn json_response<T: for<'de> serde::Deserialize<'de>>(
        &self,
        response: reqwest::Response,
    ) -> ClientResult<T> {
        let text = self.response_text(response).await?;
        Ok(from_json_str(text.as_str())?)
    }

    pub async fn json_errcode_field<T: for<'de> serde::Deserialize<'de>>(
        &self,
        response: reqwest::Response,
        field_name: &str,
    ) -> ClientResult<T> {
        let json: serde_json::Value = self.json_response(response).await?;
        self.json_check_errcode(&json)?;
        let value = json.get(field_name);
        if let Some(value) = value {
            let data = from_json_value(&value)?;
            Ok(data)
        } else {
            Err(ClientError::error_message(format!("未找到{}", field_name)))
        }
    }

    pub async fn json_errcode_else<T: for<'de> serde::Deserialize<'de>>(
        &self,
        response: reqwest::Response,
    ) -> ClientResult<T> {
        let mut json: serde_json::Value = self.json_response(response).await?;
        self.json_check_errcode(&json)?;
        // 未进行强校验, check_errcode 必为对象
        if let Some(map) = json.as_object_mut() {
            map.remove("errcode");
            map.remove("errmsg");
        }
        let data = from_json_value(&json)?;
        Ok(data)
    }

    pub async fn json_code_field<T: for<'de> serde::Deserialize<'de>>(
        &self,
        response: reqwest::Response,
        field_name: &str,
    ) -> ClientResult<T> {
        let json: serde_json::Value = self.json_response(response).await?;
        self.json_check_code(&json)?;
        let value = json.get(field_name);
        if let Some(value) = value {
            let data = from_json_value(&value)?;
            Ok(data)
        } else {
            Err(ClientError::error_message(format!("未找到{}", field_name)))
        }
    }

    fn json_check_errcode(&self, json: &serde_json::Value) -> ClientResult<()> {
        if let Some(code) = json.get("errcode") {
            if code.is_i64() {
                let code = code.as_i64().unwrap();
                if code == 0 {
                    Ok(())
                } else {
                    let errmsg = json.get("errmsg");
                    if let Some(errmsg) = errmsg {
                        if errmsg.is_string() {
                            return Err(ClientError::server_error(
                                code,
                                errmsg.as_str().unwrap_or("").to_string(),
                            ));
                        }
                    }
                    Err(ClientError::server_error(code, "".to_string()))
                }
            } else {
                Err(ClientError::error_message("errcode不是整数".to_string()))
            }
        } else {
            Err(ClientError::error_message("未找到errcode".to_string()))
        }
    }

    fn json_check_code(&self, json: &serde_json::Value) -> ClientResult<()> {
        if let Some(code) = json.get("code") {
            if code.is_i64() {
                let code = code.as_i64().unwrap();
                if code == 0 {
                    Ok(())
                } else {
                    let errmsg = json.get("msg");
                    if let Some(errmsg) = errmsg {
                        if errmsg.is_string() {
                            return Err(ClientError::server_error(
                                code,
                                errmsg.as_str().unwrap_or("").to_string(),
                            ));
                        }
                    }
                    Err(ClientError::server_error(code, "".to_string()))
                }
            } else {
                Err(ClientError::error_message("code不是整数".to_string()))
            }
        } else {
            Err(ClientError::error_message("未找到code".to_string()))
        }
    }
}
