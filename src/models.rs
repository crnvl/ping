use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub board: String,
    pub thumb_url: String,
    pub content: String,
    pub username: String,
    pub ref_id: String,
    pub time: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserMessage {
    pub thumb_url: Option<String>,
    pub content: String,
    pub username: Option<String>,
    pub ref_id: Option<String>,
}