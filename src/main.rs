//! A simple questionaire service built for pairing/selecting people.
//! The use case is intended to be an *ABI-Zeitung* poll.
//!
//! # Usage
//!
//! ```bash
//! RUST_LOG=debug cargo run
//! ```

use actix_web::{
    App, HttpServer,
    web::{self, Data},
};
use dotenv::dotenv;
use sea_orm::Database;
use sea_orm_migration::prelude::*;
use std::{process::exit, sync::Mutex};

// use crate::database::load_data;
use config::Config;
use handlers::{
    get_api, get_authed, get_student, get_students, get_teacher, get_teachers, post_api,
};
use migrator::Migrator;

use crate::database::{load_questions, load_students, load_teachers};

mod auth;
mod config;
mod database;
mod entities;
mod handlers;
mod migrator;
mod models;

const CONFIG_FILE: &str = "config.toml";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init();

    log::info!("starting server...");

    log::info!("loading config from {CONFIG_FILE}...");
    let cfg = match Config::new() {
        Ok(c) => c,
        Err(e) => {
            log::error!("failed to load config from {CONFIG_FILE}");
            log::error!("> {e}");
            exit(1)
        }
    };

    log::info!("connecting to DB...");
    let db = match Database::connect(&cfg.db.database_url).await {
        Ok(db) => db,
        Err(e) => {
            log::error!("failed to connect to DB at {}", cfg.db.database_url);
            log::error!("> {e}");
            exit(1)
        }
    };

    log::info!("refreshing migrations");
    if let Err(e) = Migrator::refresh(&db).await {
        log::error!("failed to refresh migrations");
        log::error!("> {e}");
        exit(1);
    }

    log::info!("loading students...");
    if let Err(e) = load_students(&db, &cfg).await {
        log::error!("failed to load students: {e}");
        exit(1)
    }

    log::info!("loading teachers...");
    if let Err(e) = load_teachers(&db, &cfg).await {
        log::error!("failed to load teachers: {e}");
        exit(1)
    }

    log::info!("loading questions...");
    if let Err(e) = load_questions(&db, &cfg).await {
        log::error!("failed to load questions: {e}");
        exit(1)
    }

    // not Mutex because sea_orm DatabaseConnection is already internally thread safe
    // otherwise would need to be mutex
    let db = Data::new(db);

    log::info!("mounting routes and services...");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&db))
            .route("/api", web::get().to(get_api))
            .route("/api", web::post().to(post_api))
            .route("/api/authed", web::get().to(get_authed))
            .route("/api/students", web::get().to(get_students))
            .route("/api/students/{id}", web::get().to(get_student))
            .route("/api/teachers", web::get().to(get_teachers))
            .route("/api/teachers/{id}", web::get().to(get_teacher))
            // .route("/api/questions", web::get().to(get_questions))
            // .route("/api/question/{id}", web::get().to(get_question))
            // .route("/api/question", web::post().to(post_question))
            // .route("/api/answers", web::get().to(get_answers))
            // .route("/api/answer/{id}", web::get().to(get_answer))
            // .route("/api/answer", web::post().to(post_answer))
            .service(
                actix_files::Files::new("/", "./static")
                    .show_files_listing()
                    .index_file("index.html")
                    .use_last_modified(true),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
