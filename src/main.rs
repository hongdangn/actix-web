// src/main.rs
mod handlers;
mod models;

use actix_files as fs;
use actix_web::{web, App, HttpServer};
use handlers::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/users", web::get().to(get_users))
            .route("/users", web::post().to(add_user))
            .route("/users", web::put().to(update_user))
            .route("/users/{id}", web::delete().to(delete_user))
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
