use crate::handlers::{get_api, get_student, get_students, get_teacher, get_teachers, post_api};
use actix_web::{App, HttpServer, web};

mod database;
mod handlers;
mod models;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    log::info!("starting server...");

    HttpServer::new(|| {
        App::new()
            .route("/api", web::get().to(get_api))
            .route("/api", web::post().to(post_api))
            .route("/api/students", web::get().to(get_students))
            .route("/api/students/{id}", web::get().to(get_student))
            .route("/api/teachers", web::get().to(get_teachers))
            .route("/api/teachers/{id}", web::get().to(get_teacher))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
