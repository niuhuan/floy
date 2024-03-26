use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::num::ParseIntError;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BbsCategory {
    pub fid: String,
    pub name: String,
    pub forumlist: Vec<Forumlist>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Forumlist {
    pub fid: String,
    pub name: String,
    pub description: String,
    #[serde(deserialize_with = "fuzzy_i64")]
    pub threads: i64,
    #[serde(deserialize_with = "fuzzy_i64")]
    pub posts: i64,
    #[serde(deserialize_with = "fuzzy_i64")]
    pub todayposts: i64,
    pub lastpost: Lastpost,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lastpost {
    pub author: String,
    pub dateline: String,
    pub subject: String,
    pub tid: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThreadPage {
    pub total: i64,
    pub page: i64,
    pub threadlist: Vec<ThreadInList>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThreadInList {
    pub tid: String,
    pub fid: String,
    pub author: String,
    pub authorid: String,
    pub author_groupid: String,
    pub author_grouptitle: String,
    pub author_medal: Vec<Medal>,
    pub avatar: String,
    pub subject: String,
    pub dateline: String,
    pub views: String,
    pub replies: String,
    pub lastpost: String,
    pub lastposter: String,
    pub status: String,
    pub displayorder: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Medal {
    pub medalid: String,
    pub name: String,
    pub image: String,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThreadData {
    pub total: i64,
    pub perpage: i64,
    pub page: i64,
    // pub sticklist: Vec<Value>,
    pub thread: ThreadDetail,
    pub postlist: Vec<Postlist>,
    pub errcode: i64,
    pub errmsg: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThreadDetail {
    pub tid: String,
    pub fid: String,
    pub author: String,
    pub authorid: String,
    pub subject: String,
    pub views: i64,
    pub replies: i64,
    pub attachment: String,
    pub recommend_add: String,
    pub recommend_sub: String,
    pub favtimes: String,
    pub sharetimes: String,
    pub relay: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Postlist {
    pub pid: String,
    pub is_app: String,
    pub fid: String,
    pub tid: String,
    pub first: String,
    pub author: String,
    pub authorid: String,
    pub avatar: String,
    pub subject: String,
    pub dateline: String,
    pub message: String,
    // pub attachments: Vec<Value>,
    // pub comments: Vec<Value>,
    pub ratelog: Vec<Ratelog>,
    pub medals: Vec<Medal>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ratelog {
    pub username: String,
    pub score: i64,
    pub reason: String,
}

fn fuzzy_i64<'de, D>(d: D) -> std::result::Result<i64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value: serde_json::Value = serde::Deserialize::deserialize(d)?;
    if value.is_i64() {
        Ok(value.as_i64().unwrap())
    } else if value.is_string() {
        let str = value.as_str().unwrap();
        let from: std::result::Result<i64, ParseIntError> = std::str::FromStr::from_str(str);
        match from {
            Ok(from) => Ok(from),
            Err(_) => Err(serde::de::Error::custom("parse error")),
        }
    } else {
        Err(serde::de::Error::custom("type error"))
    }
}
