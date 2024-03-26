use crate::client::community::dataobject::{BbsCategory, ThreadData, ThreadPage};
use crate::client::types::ClientResult;
use crate::client::Client;
use serde_json::json;

pub mod dataobject;

#[cfg(test)]
mod tests;

impl Client {
    pub async fn bbs_categories(&self) -> ClientResult<Vec<BbsCategory>> {
        let rsp = self
            .bbs_get(
                "/apicloud/",
                json!({
                    "mod": "index",
                }),
            )
            .await?;
        let data = self.json_errcode_field(rsp, "catlist").await?;
        Ok(data)
    }

    pub async fn bbs_threads_list(&self, fid: &str, page: i64) -> ClientResult<ThreadPage> {
        let rsp = self
            .bbs_post(
                "/apicloud/",
                json!({
                    "mod": "forumdisplay",
                }),
                json!({
                    "fid": fid,
                    "page": page,
                }),
            )
            .await?;
        let data = self.json_errcode_else(rsp).await?;
        Ok(data)
    }

    pub async fn bbs_thread(&self, tid: &str, page: i64) -> ClientResult<ThreadData> {
        let rsp = self
            .bbs_post(
                "/apicloud/",
                json!({
                    "mod": "viewthread",
                }),
                json!({
                    "tid": tid,
                    "page": page,
                }),
            )
            .await?;
        let data = self.json_errcode_else(rsp).await?;
        Ok(data)
    }
}
