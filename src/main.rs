use actix_cors::Cors;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use api::{
    boards::get_boards,
    debug::{index, stats},
    messages::{create_post, get_comments, get_post, get_posts},
};
use std::path::Path;

mod api;
mod models;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(|| {
        let cors = Cors::default().send_wildcard().allow_any_origin().allow_any_method().allow_any_header();

        let mut create_tables = "";
        if !Path::new("./data/main.db").exists() {
            create_tables = "CREATE TABLE messages (id INTEGER, board TEXT DEFAULT 'all', thumb_url TEXT DEFAULT '', content TEXT, username TEXT DEFAULT 'anonymous', ref_id INTEGER DEFAULT 0, time DATETIME DEFAULT CURRENT_TIMESTAMP);"
        }

        let connection = sqlite::open("./data/main.db").unwrap();
        connection.execute(create_tables).unwrap();

        let data = Data::new(connection);

        let logger = Logger::default();
        App::new().wrap(logger).wrap(cors)
        .app_data(data)
        .service(index)
        .service(stats)
        .service(get_posts)
        .service(get_boards)
        .service(get_post)
        .service(get_comments)
        .service(create_post)
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
