use std::path::Path;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("SQLite test!");

    let mut create_tables = "";
    if !Path::new("main.db").exists() {
        create_tables = "CREATE TABLE messages (content TEXT, username TEXT DEFAULT 'anonymous', time DATETIME DEFAULT CURRENT_TIMESTAMP);"
    }

    let connection = sqlite::open("main.db").unwrap();
    connection.execute(create_tables).unwrap();

    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");

    HttpServer::new(|| App::new().app_data(connection))
        .bind(("127.0.0.1", 80))?
        .run()
        .await
}
