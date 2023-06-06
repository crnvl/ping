use actix_web::{get, Responder, HttpResponse};

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Learn more: https://github.com/angelsflyinhell/ping")
}