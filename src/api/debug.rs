use actix_web::{
    get,
    web::{Json},
};

#[get("/")]
pub async fn index() -> Json<String> {
    Json("Barebones, anonymous chat platform as a web server using SQLite and Rust. Learn more: https://github.com/angelsflyinhell/ping".to_string())
}
