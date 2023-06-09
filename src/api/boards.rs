use actix_web::{web::{Data, Json}, get};
use serde::{Serialize, Deserialize};
use sqlite::Connection;

#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    name: String,
    size: i64,
}

#[get("/boards")]
pub async fn get_boards(db: Data<Connection>) -> Json<Vec<Board>> {
    let query = "SELECT DISTINCT board FROM messages";
    let mut statement = db.prepare(query).unwrap();

    let mut boards = Vec::new();
    while let sqlite::State::Row = statement.next().unwrap() {
        let board: String = statement.read::<String, _>(0).unwrap();
        boards.push(board);
    }

    let mut boards_with_size = Vec::new();
    for board in boards {
        let query = format!("SELECT COUNT(*) FROM messages WHERE board = '{}'", board);
        let mut statement = db.prepare(&query).unwrap();
        statement.next().unwrap();
        let size = statement.read::<i64, _>(0).unwrap();
        boards_with_size.push(Board { name: board, size });
    }

    Json(boards_with_size)
}