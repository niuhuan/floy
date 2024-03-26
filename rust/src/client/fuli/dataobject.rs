use crate::client::types::enum_int_str;
use serde_derive::Deserialize;
use serde_derive::Serialize;

enum_int_str!(BannerPosition{
    HOMEPAGE(1,"HOMEPAGE"),
    STARTUP(2,"STARTUP"),
});

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Banner {
    pub id: String,
    pub name: String,
    pub img: String,
    pub start_time: String,
    pub end_time: String,
    pub link_type: String,
    pub tid: String,
    pub link: String,
    pub status: String,
    pub addtime: String,
    pub updatetime: String,
    pub orderby: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Category {
    pub id: i64,
    pub count: i64,
    pub description: String,
    pub link: String,
    pub name: String,
    pub slug: String,
    pub taxonomy: String,
    pub parent: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Post {
    pub id: i64,
    pub date: String,
    pub title: Title,
    #[serde(default)]
    pub excerpt: Content,
    #[serde(default)]
    pub content: Content,
    pub comment_status: String,
    pub categories: Vec<i64>,
    pub tags: Vec<i64>,
    pub acf: Acf,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Title {
    pub rendered: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Content {
    pub rendered: String,
    pub protected: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Acf {
    pub img: Option<String>,
    pub content: Option<String>,
}
