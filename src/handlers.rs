use actix_web::{
    HttpRequest, HttpResponse, Result,
    error::*,
    web::{self, Data},
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use std::sync::Mutex;

use crate::{
    auth::auth,
    entities::{self, prelude::*},
    models::*,
};

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
    log::debug!("{}", message.message);
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

    let mut sts = Vec::new();

    match &query.email {
        Some(m) => {
            sts = students
                .iter()
                .filter(|s| &s.email == m)
                .map(|s| student::StudentSafe::from_model(s))
                .collect();

            if sts.is_empty() {
                return Err(ErrorNotFound("failed to find students with email: {m}"));
            }
        }
        None => {
            sts = students
                .iter()
                .map(|s| student::StudentSafe::from_model(s))
                .collect();

            if sts.is_empty() {
                return Err(ErrorNotFound("failed to find students"));
            }
        }
    }

    Ok(HttpResponse::Ok().json(sts))
}

// get student data by id
pub async fn get_student(
    db: Data<DatabaseConnection>,
    req: HttpRequest,
    sid: web::Path<i32>,
) -> Result<HttpResponse> {
    auth(&db, &req).await?;

    let sid = sid.into_inner();

    let s = Student::find_by_id(sid)
        .one(db.as_ref())
        .await
        .map_err(|e| ErrorInternalServerError(e))?
        .ok_or(ErrorNotFound(format!(
            "failed to find student via id: {sid}"
        )))?;

    let s = student::StudentSafe::from_model(&s);

    Ok(HttpResponse::Ok().json(s))
}

// get a list of teachers
pub async fn get_teachers(db: Data<DatabaseConnection>, req: HttpRequest) -> Result<HttpResponse> {
    auth(&db, &req).await?;

    let ts: Vec<teacher::Teacher> = Teacher::find()
        .all(db.get_ref())
        .await
        .map_err(|e| ErrorInternalServerError(e))?
        .iter()
        .map(|t| teacher::Teacher::from_model(t))
        .collect();

    Ok(HttpResponse::Ok().json(ts))
}

// get teacher data by id
pub async fn get_teacher(
    db: Data<DatabaseConnection>,
    req: HttpRequest,
    tid: web::Path<i32>,
) -> Result<HttpResponse> {
    auth(&db, &req).await?;

    let tid = tid.into_inner();

    let q = Teacher::find_by_id(tid)
        .one(db.as_ref())
        .await
        .map_err(|e| ErrorInternalServerError(e))?
        .ok_or(ErrorNotFound(format!(
            "failed to find teacher via id: {tid}"
        )))?;

    let q = teacher::Teacher::from_model(&q);

    Ok(HttpResponse::Ok().json(q))
}

// get a list of questions
pub async fn get_questions(db: Data<DatabaseConnection>, req: HttpRequest) -> Result<HttpResponse> {
    auth(&db, &req).await?;
    let qs: Vec<question::Question> = Question::find()
        .all(db.get_ref())
        .await
        .map_err(|e| ErrorInternalServerError(e))?
        .iter()
        .map(|q| question::Question::from_model(q))
        .collect();

    Ok(HttpResponse::Ok().json(qs))
}

// get question indo by id
pub async fn get_question(
    db: Data<DatabaseConnection>,
    req: HttpRequest,
    qid: web::Path<i32>,
) -> Result<HttpResponse> {
    auth(&db, &req).await?;

    let qid = qid.into_inner();

    let q = Question::find_by_id(qid)
        .one(db.as_ref())
        .await
        .map_err(|e| ErrorInternalServerError(e))?
        .ok_or(ErrorNotFound(format!(
            "failed to find question via id: {qid}"
        )))?;

    let q = question::Question::from_model(&q);

    Ok(HttpResponse::Ok().json(q))
}

// get a list of answers
pub async fn get_answers(db: Data<DatabaseConnection>, req: HttpRequest) -> Result<HttpResponse> {
    auth(&db, &req).await?;
    let ans: Vec<answer::Answer> = Answer::find()
        .all(db.get_ref())
        .await
        .map_err(|e| ErrorInternalServerError(e))?
        .iter()
        .map(|a| answer::Answer::from_model(a))
        .collect();

    Ok(HttpResponse::Ok().json(ans))
}

// get answer info by id
pub async fn get_answer(
    db: Data<DatabaseConnection>,
    req: HttpRequest,
    aid: web::Path<i32>,
) -> Result<HttpResponse> {
    auth(&db, &req).await?;

    let aid = aid.into_inner();

    let a = Question::find_by_id(aid)
        .one(db.as_ref())
        .await
        .map_err(|e| ErrorInternalServerError(e))?
        .ok_or(ErrorNotFound(format!(
            "failed to find answer via id: {aid}"
        )))?;

    let a = question::Question::from_model(&a);

    Ok(HttpResponse::Ok().json(a))
}

// post answer data to db
pub async fn post_answer(
    db: Data<DatabaseConnection>,
    req: HttpRequest,
    answer: web::Json<answer::NewAnswer>,
) -> Result<HttpResponse> {
    auth(&db, &req).await?;

    let answer = answer.into_inner();

    // get all existing answers
    let ans: Vec<answer::Answer> = Answer::find()
        .all(db.as_ref())
        .await
        .map_err(|e| ErrorInternalServerError(e))?
        .iter()
        .map(|a| answer::Answer::from_model(a))
        .collect();

    // find answers of user with sid to question with qid
    let ans: Vec<&answer::Answer> = ans
        .iter()
        .filter(|a| a.sid == answer.sid)
        .filter(|a| a.qid == answer.qid)
        .collect();

    if ans.len() == 0 {
        let an = entities::answer::ActiveModel {
            sid: Set(answer.sid),
            qid: Set(answer.qid),
            opt1: Set(answer.opt1),
            opt2: Set(answer.opt2),
            opt3: Set(answer.opt3),
            ..Default::default()
        };

        an.insert(db.as_ref())
            .await
            .map_err(|e| ErrorInternalServerError(e))?;
    } else if ans.len() == 1 {
        let an = ans[0];
        let mut an = entities::answer::ActiveModel {
            id: Set(an.id),
            sid: Set(an.sid),
            qid: Set(an.qid),
            opt1: Set(answer.opt1),
            opt2: Set(answer.opt2),
            opt3: Set(answer.opt3),
            ..Default::default()
        };

        an.update(db.as_ref())
            .await
            .map_err(|e| ErrorInternalServerError(e))?;
    } else {
        return Err(ErrorInternalServerError("DB has conflicting entries"));
    }

    Ok(HttpResponse::Ok().finish())
}
