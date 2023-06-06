use actix_web::{get, Responder, HttpResponse};

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Barebones, anonymous chat platform as a web server using SQLite and Rust.\nLearn more: https://github.com/angelsflyinhell/ping")
}