use crate::database;
use crate::database::active::cookie::CookieModel;
use bytes::Bytes;
use cookie::Cookie;
use reqwest::header::HeaderValue;
use reqwest::Url;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::slice::Iter;

fn cookie_key(cookie_model: &CookieModel) -> String {
    format!(
        "{}<->{}<->{}",
        cookie_model.domain, cookie_model.path, cookie_model.name
    )
}

#[derive(Debug)]
pub struct CookieJar {
    map: tokio::sync::Mutex<HashMap<String, CookieModel>>,
    mutex: tokio::sync::Mutex<()>,
}

impl CookieJar {
    pub fn new(model: Vec<CookieModel>) -> Self {
        Self {
            map: tokio::sync::Mutex::new({
                let mut map = HashMap::new();
                for x in model {
                    map.insert(cookie_key(&x), x);
                }
                map
            }),
            mutex: tokio::sync::Mutex::new(()),
        }
    }
}

impl CookieJar {
    async fn update_cookies_in_memory(&self, cookies: &Vec<CookieModel>) {
        let mut lock = self.map.lock().await;
        let map = lock.deref_mut();
        for x in cookies {
            map.insert(cookie_key(x), x.clone());
        }
    }

    async fn update_cookies_in_database(&self, cookies: &Vec<CookieModel>) {
        let _lock = self.mutex.lock().await;
        let _ = database::active::cookie::update_cookies(cookies).await;
    }

    fn parse_need_update_cookies(
        &self,
        cookie_headers: Iter<'_, HeaderValue>,
        url: &Url,
    ) -> Vec<CookieModel> {
        let mut update_cookies = vec![];
        for val in cookie_headers {
            let s = val.to_str().unwrap();
            let cookie = Cookie::parse(s);
            if let Ok(cookie) = cookie {
                let domain = if let Some(domain) = cookie.domain() {
                    domain.to_string()
                } else if let Some(host) = url.host_str() {
                    host.to_string()
                } else {
                    continue;
                };
                let path = if let Some(path) = cookie.path() {
                    path.to_string()
                } else {
                    "/".to_string()
                };
                let name = cookie.name();
                let value = cookie.value();
                let expires = if let Some(expires) = cookie.expires() {
                    if let Some(expires) = expires.datetime() {
                        expires.unix_timestamp()
                    } else {
                        -1
                    }
                } else {
                    -1
                };
                update_cookies.push(CookieModel {
                    domain,
                    path,
                    name: name.to_string(),
                    value: value.to_string(),
                    expires,
                });
            }
        }
        update_cookies
    }
}

impl CookieJar {
    pub async fn set_cookies(&self, cookie_headers: Vec<HeaderValue>, url: &Url) {
        let update_cookies = self.parse_need_update_cookies(cookie_headers.iter(), url);
        self.update_cookies_in_memory(&update_cookies).await;
        self.update_cookies_in_database(&update_cookies).await;
    }

    pub async fn cookies(&self, _url: &Url) -> Option<HeaderValue> {
        let lock = self.map.lock().await;
        let map = lock.deref();
        let s = map
            .iter()
            .enumerate()
            .map(|(_, (_, cookie))| format!("{}={}", cookie.name, cookie.value))
            .collect::<Vec<_>>()
            .join("; ");
        HeaderValue::from_maybe_shared(Bytes::from(s)).ok()
    }
}
