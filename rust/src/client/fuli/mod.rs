use crate::client::types::ClientResult;
use crate::client::Client;
use dataobject::*;
use serde_json::json;

pub(crate) mod dataobject;

#[cfg(test)]
mod tests;

impl Client {
    pub async fn banners(&self, banner_position: BannerPosition) -> ClientResult<Vec<Banner>> {
        let rsp = self
            .bbs_get(
                "/apicloud/fuliba/api.php",
                json!({
                    "action": "banner_list",
                    "type": TryInto::<i64>::try_into(banner_position)?,
                }),
            )
            .await?;
        let data = self.json_code_field(rsp, "data").await?;
        Ok(data)
    }

    pub async fn categories(&self) -> ClientResult<Vec<Category>> {
        let rsp = self
            .api_get::<()>("/wp-json/wp/v2/categories", None)
            .await?;
        let data = self.json_response(rsp).await?;
        Ok(data)
    }

    pub async fn posts(&self, page: i64, category: Option<i64>) -> ClientResult<Vec<Post>> {
        let mut json = json!({
            "_fields": "id,date,title,excerpt,categories,tags,acf,comment_status",
            "page": page,
        });
        if let Some(category) = category {
            json.as_object_mut()
                .unwrap()
                .insert("categories".to_string(), json!(category));
        };
        let rsp = self.api_get("/wp-json/wp/v2/posts", json).await?;
        let data = self.json_response(rsp).await?;
        Ok(data)
    }

    pub async fn post(&self, id: i64) -> ClientResult<Post> {
        let rsp = self
            .api_get(
                format!("/wp-json/wp/v2/posts/{id}").as_str(),
                json!({
                    "_fields": "id,date,link,title,content,comment_status,categories,tags,acf",
                }),
            )
            .await?;
        let data = self.json_response(rsp).await?;
        Ok(data)
    }

    pub async fn comments(&self, post_id: i64, page: i64) -> ClientResult<serde_json::Value> {
        let rsp = self
            .api_get(
                "/wp-json/wp/v2/comments".to_string().as_str(),
                json!({
                    "_fields": "id,parent,author_name,date,content,author_avatar_urls",
                    "post": post_id,
                    "page": page,
                }),
            )
            .await?;
        let data = self.json_response(rsp).await?;
        Ok(data)
    }
}
