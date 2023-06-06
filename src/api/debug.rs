use actix_web::{
    get,
    web::{Data, Json},
};
use sqlite::Connection;

#[get("/")]
pub async fn index() -> Json<String> {
    Json("Barebones, anonymous chat platform as a web server using SQLite and Rust.\nLearn more: https://github.com/angelsflyinhell/ping".to_string())
}

#[get("/dbg/insert_sample_data")]
pub async fn insert_sample_data(db: Data<Connection>) -> Json<String> {
    db.execute("INSERT INTO messages (content) VALUES ('Hello, world!');")
        .unwrap();
    Json("Inserted sample data.".to_string())
}
