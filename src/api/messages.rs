use actix_web::{
    get, post,
    web::{self, Data, Json},
};
use sqlite::{Connection, State};

use crate::{
    models::{Message, UserMessage},
    utils::generate_snowflake,
};

#[get("/posts/{board}")]
pub async fn get_posts(path: web::Path<String>, db: Data<Connection>) -> Json<Vec<Message>> {
    let board = path.into_inner();

    let query = format!("SELECT * FROM messages WHERE board = '{}'", board);
    let mut statement = db.prepare(query).unwrap();

    let mut posts = Vec::new();
    while let State::Row = statement.next().unwrap() {
        posts.push(Message {
            id: generate_snowflake(),
            board: statement.read::<String, _>(0).unwrap(),
            thumb_url: statement.read::<String, _>(1).unwrap(),
            content: statement.read::<String, _>(2).unwrap(),
            username: statement.read::<String, _>(3).unwrap(),
            ref_id: statement.read::<i64, _>(4).unwrap(),
            time: statement.read::<String, _>(5).unwrap(),
        });
    }

    Json(posts)
}

#[post("/posts/{board}")]
pub async fn create_post(
    path: web::Path<String>,
    db: Data<Connection>,
    body: Json<UserMessage>,
) -> Json<String> {
    let board = path.into_inner();
    let message = body.into_inner();

    let query = format!("INSERT INTO messages (board, thumb_url, content, username, ref_id) VALUES ('{}', '{}', '{}', '{}', '{}')", 
        board, 
        message.thumb_url.unwrap_or("".to_string()), 
        message.content, 
        message.username.unwrap_or("anonymous".to_string()), 
        message.ref_id.unwrap_or(0)
    );

    match db.execute(query) {
        Ok(_) => (),
        Err(_) => return Json("Error creating post.".to_string()),
    }

    Json("Post created.".to_string())
}
