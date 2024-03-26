use crate::client::fuli::dataobject::{Category, Post};
use crate::client::types::ClientResult;
use crate::database::cache::web_cache::{cache_first, CacheCross};
use crate::domain::defines::get_client;
use async_trait::async_trait;

pub async fn fuli_categories() -> anyhow::Result<Vec<Category>> {
    cache_first(
        "fuli_categories".to_string(),
        std::time::Duration::from_secs(60 * 10),
        Box::new(FoliCategoriesLoader {}),
    )
    .await
}

struct FoliCategoriesLoader {}

#[async_trait]
impl CacheCross<Vec<Category>> for FoliCategoriesLoader {
    async fn load(&self) -> ClientResult<Vec<Category>> {
        get_client().categories().await
    }
}

pub async fn fuli_posts(page: i64, category_id: Option<i64>) -> anyhow::Result<Vec<Post>> {
    Ok(get_client().posts(page, category_id).await?)
}

pub async fn fuli_post(post_id: i64) -> anyhow::Result<Post> {
    cache_first(
        format!("fuli_post_{}", post_id),
        std::time::Duration::from_secs(60 * 20),
        Box::new(FoliPostLoader { post_id }),
    )
    .await
}

struct FoliPostLoader {
    post_id: i64,
}

#[async_trait]
impl CacheCross<Post> for FoliPostLoader {
    async fn load(&self) -> ClientResult<Post> {
        get_client().post(self.post_id).await
    }
}
