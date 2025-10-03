use actix_web::{
    HttpRequest, HttpResponse, Result,
    error::*,
    web::{self, Data},
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::sync::Mutex;

use crate::auth::auth;
use crate::entities::prelude::*;
use crate::models::{PostMessage, student, teacher};

/*
// function template

pub async fn xyz(db: Data<DatabaseConnection>) -> Result<HttpResponse> {
    ...
    Ok(HttpResponse::Ok().finish())
}
*/

// API GET ping
pub async fn get_api() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json("Hello, World!"))
}

// API POST ping
pub async fn post_api(message: web::Json<PostMessage>) -> Result<HttpResponse> {
    let message = message.into_inner();
    println!("{}", message.message);
    Ok(HttpResponse::Ok().json("message received"))
}

// check if auth data is valid
pub async fn get_authed(db: Data<DatabaseConnection>, req: HttpRequest) -> Result<HttpResponse> {
    auth(&db, &req).await?;

    Ok(HttpResponse::Ok().finish())
}

// get a list of students
// optionally filter for a specific email adress
pub async fn get_students(
    db: Data<DatabaseConnection>,
    req: HttpRequest,
    query: web::Query<student::StudentsQuery>,
) -> Result<HttpResponse> {
    auth(&db, &req).await?;

    let students = Student::find()
        .all(db.get_ref())
        .await
        .map_err(|e| ErrorInternalServerError(e))?;

    let mut res = Vec::new();

    match &query.email {
        Some(m) => {
            res = students
                .iter()
                .filter(|s| &s.email == m)
                .map(|s| student::StudentSafe::from_model(s))
                .collect();

            if res.is_empty() {
                return Err(ErrorNotFound("failed to find students with email: {m}"));
            }
        }
        None => {
            res = students
                .iter()
                .map(|s| student::StudentSafe::from_model(s))
                .collect();

            if res.is_empty() {
                return Err(ErrorNotFound("failed to find students"));
            }
        }
    }

    Ok(HttpResponse::Ok().json(res))
}

// get student data by id
pub async fn get_student(
    db: Data<DatabaseConnection>,
    req: HttpRequest,
    sid: web::Path<i32>,
) -> Result<HttpResponse> {
    auth(&db, &req).await?;

    let sid = sid.into_inner();

    let res = Student::find_by_id(sid)
        .one(db.as_ref())
        .await
        .map_err(|e| ErrorInternalServerError(e))?
        .ok_or(ErrorNotFound(format!(
            "failed to find student via id: {sid}"
        )))?;

    let res = student::StudentSafe::from_model(&res);

    Ok(HttpResponse::Ok().json(res))
}

// get a list of techers
pub async fn get_teachers(db: Data<DatabaseConnection>, req: HttpRequest) -> Result<HttpResponse> {
    auth(&db, &req).await?;

    let res: Vec<teacher::Teacher> = Teacher::find()
        .all(db.get_ref())
        .await
        .map_err(|e| ErrorInternalServerError(e))?
        .iter()
        .map(|t| teacher::Teacher::from_model(t))
        .collect();

    Ok(HttpResponse::Ok().json(res))
}

// get teacher data by id
pub async fn get_teacher(
    db: Data<DatabaseConnection>,
    req: HttpRequest,
    tid: web::Path<i32>,
) -> Result<HttpResponse> {
    auth(&db, &req).await?;

    let tid = tid.into_inner();

    let res = Teacher::find_by_id(tid)
        .one(db.as_ref())
        .await
        .map_err(|e| ErrorInternalServerError(e))?
        .ok_or(ErrorNotFound(format!(
            "failed to find teacher via id: {tid}"
        )))?;

    let res = teacher::Teacher::from_model(&res);

    Ok(HttpResponse::Ok().json(res))
}
