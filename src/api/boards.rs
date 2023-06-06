use actix_web::{web::{Data, Json}, get};
use sqlite::Connection;

#[get("/boards")]
pub async fn get_boards(db: Data<Connection>) -> Json<Vec<String>> {
    let query = "SELECT DISTINCT board FROM messages";
    let mut statement = db.prepare(query).unwrap();

    let mut boards = Vec::new();
    while let sqlite::State::Row = statement.next().unwrap() {
        let board: String = statement.read::<String, _>(0).unwrap();
        boards.push(board);
    }

    Json(boards)
}