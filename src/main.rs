use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use std::path::Path;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(|| {
        let mut create_tables = "";
        if !Path::new("main.db").exists() {
            create_tables = "CREATE TABLE messages (thumb_url TEXT DEFAULT '', content TEXT, username TEXT DEFAULT 'anonymous', time DATETIME DEFAULT CURRENT_TIMESTAMP);"
        }

        let connection = sqlite::open("main.db").unwrap();
        connection.execute(create_tables).unwrap();

        let data = Data::new(connection);

        let logger = Logger::default();
        App::new().wrap(logger)
        .app_data(data)
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
