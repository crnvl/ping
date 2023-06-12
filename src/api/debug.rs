use actix_web::{
    get,
    web::{Data, Json},
};
use sqlite::Connection;

#[get("/")]
pub async fn index() -> Json<String> {
    Json("Barebones, anonymous chat platform as a web server using SQLite and Rust. Learn more: https://github.com/angelsflyinhell/ping. (v1.0.0)".to_string())
}

#[get("/stats")]
pub async fn stats(db: Data<Connection>) -> Json<String> {
    let mut stmt = db.prepare("SELECT COUNT(*) FROM messages").unwrap();
    stmt.next().unwrap();
    let count = stmt.read::<i64, _>(0).unwrap();

    let mut stmt = db.prepare("SELECT COUNT(DISTINCT board) FROM messages").unwrap();
    stmt.next().unwrap();
    let boards = stmt.read::<i64, _>(0).unwrap();

    Json(format!("{} messages across {} boards", count, boards))
}
