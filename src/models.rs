use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: i64,
    pub board: String,
    pub thumb_url: String,
    pub content: String,
    pub username: String,
    pub ref_id: i64,
    pub time: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserMessage {
    pub thumb_url: Option<String>,
    pub content: String,
    pub username: Option<String>,
    pub ref_id: Option<i64>,
}