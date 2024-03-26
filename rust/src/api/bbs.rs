use crate::client::community::dataobject::{BbsCategory, ThreadData, ThreadPage};
use crate::client::types::ClientResult;
use crate::database::cache::web_cache::{cache_first, CacheCross};
use crate::domain::defines::get_client;
use async_trait::async_trait;
use std::time::Duration;

pub async fn bbs_categories() -> anyhow::Result<Vec<BbsCategory>> {
    let key = "bbs_categories".to_string();
    cache_first(
        key,
        Duration::from_secs(60 * 10),
        Box::new(BbsCategoryLoader {}),
    )
    .await
}

struct BbsCategoryLoader {}

#[async_trait]
impl CacheCross<Vec<BbsCategory>> for BbsCategoryLoader {
    async fn load(&self) -> ClientResult<Vec<BbsCategory>> {
        get_client().bbs_categories().await
    }
}

pub async fn bbs_threads_list(fid: String, page: i64) -> anyhow::Result<ThreadPage> {
    Ok(get_client().bbs_threads_list(fid.as_str(), page).await?)
}

pub async fn bbs_thread(tid: String, page: i64) -> anyhow::Result<ThreadData> {
    Ok(get_client().bbs_thread(tid.as_str(), page).await?)
}
