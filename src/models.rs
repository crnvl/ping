use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub board: String,
    pub thumb_url: String,
    pub content: String,
    pub username: String,
    pub time: String,
}