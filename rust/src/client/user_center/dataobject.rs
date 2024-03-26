use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    pub uid: String,
    pub email: String,
    pub username: String,
    pub avatar: String,
    pub friends: i64,
    pub posts: String,
    pub threads: String,
    pub extcredits1: String,
    pub extcredits2: String,
    pub extcredits3: String,
    pub extcredits4: String,
    pub extcredits5: String,
    pub extcredits6: String,
    pub extcredits7: String,
    pub extcredits8: String,
    pub oltime: String,
    pub follower: String,
    pub following: String,
    pub sharings: String,
    pub regdate: String,
    pub regip: String,
    pub regip_loc: String,
    pub lastip: String,
    pub lastip_loc: String,
    pub lastvisit: String,
    pub credits: String,
    pub groupid: String,
    pub grouptitle: String,
    pub onlinestatus: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Login {
    pub uid: i64,
    pub email: String,
    pub hash: String,
    #[serde(rename = "S5r8_2132_saltkey")]
    pub s5r8_2132_saltkey: String,
    #[serde(rename = "S5r8_2132_auth")]
    pub s5r8_2132_auth: String,
}
