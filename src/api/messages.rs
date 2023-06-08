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
            id: statement.read::<String, _>(0).unwrap(),
            board: statement.read::<String, _>(1).unwrap(),
            thumb_url: statement.read::<String, _>(2).unwrap(),
            content: statement.read::<String, _>(3).unwrap(),
            username: statement.read::<String, _>(4).unwrap(),
            ref_id: statement.read::<String, _>(5).unwrap(),
            time: statement.read::<String, _>(6).unwrap(),
        });
    }

    Json(posts)
}

#[get("/post/{id}")]
pub async fn get_post(path: web::Path<i64>, db: Data<Connection>) -> Json<Message> {
    let id = path.into_inner();

    let query = format!("SELECT * FROM messages WHERE id = '{}'", id);
    let mut statement = db.prepare(query).unwrap();

    statement.next().unwrap();

    let post = match statement.read::<String, _>(3) {
        Ok(_) => Message {
            id: statement.read::<String, _>(0).unwrap(),
            board: statement.read::<String, _>(1).unwrap(),
            thumb_url: statement.read::<String, _>(2).unwrap(),
            content: statement.read::<String, _>(3).unwrap(),
            username: statement.read::<String, _>(4).unwrap(),
            ref_id: statement.read::<String, _>(5).unwrap(),
            time: statement.read::<String, _>(6).unwrap(),
        },
        Err(_) => return Json(Message {
            id: "0".to_string(),
            board: "".to_string(),
            thumb_url: "".to_string(),
            content: "Post not found.".to_string(),
            username: "".to_string(),
            ref_id: "0".to_string(),
            time: "".to_string(),
        }),
    };

    Json(post)
}

#[get("/post/{id}/comments")]
pub async fn get_comments(path: web::Path<i64>, db: Data<Connection>) -> Json<Vec<Message>> {
    let id = path.into_inner();

    let query = format!("SELECT * FROM messages WHERE ref_id = '{}'", id);
    let mut statement = db.prepare(query).unwrap();

    let mut posts = Vec::new();
    while let State::Row = statement.next().unwrap() {
        posts.push(Message {
            id: statement.read::<String, _>(0).unwrap(),
            board: statement.read::<String, _>(1).unwrap(),
            thumb_url: statement.read::<String, _>(2).unwrap(),
            content: statement.read::<String, _>(3).unwrap(),
            username: statement.read::<String, _>(4).unwrap(),
            ref_id: statement.read::<String, _>(5).unwrap(),
            time: statement.read::<String, _>(6).unwrap(),
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

    let query = format!("INSERT INTO messages (id, board, thumb_url, content, username, ref_id) VALUES ('{}', '{}', '{}', '{}', '{}', '{}')", 
        generate_snowflake(),
        board, 
        message.thumb_url.unwrap_or("".to_string()), 
        message.content.replace("'", "''"),
        message.username.unwrap_or("anonymous".to_string()), 
        message.ref_id.unwrap_or("0".to_string())
    );

    println!("{}", query);

    match db.execute(query) {
        Ok(_) => (),
        Err(err) => return Json(format!("Error creating post: {}", err)),
    }

    Json("Post created.".to_string())
}
