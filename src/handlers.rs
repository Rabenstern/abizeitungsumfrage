use actix_web::{HttpRequest, HttpResponse, Result, web};
use diesel::prelude::*;

use crate::auth::auth;
use crate::database::establish_connection;
use crate::models::{PostMessage, Student, StudentSafe, StudentsQuery, Teacher};

pub async fn get_api() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json("Hello, World!"))
}

pub async fn post_api(message: web::Json<PostMessage>) -> Result<HttpResponse> {
    let message = message.into_inner();
    println!("{}", message.message);
    Ok(HttpResponse::Ok().json("message received"))
}

pub async fn get_students(
    req: HttpRequest,
    query: web::Query<StudentsQuery>,
) -> Result<HttpResponse> {
    let authed = auth(&req);
    if authed.is_err() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    use crate::schema::student::dsl::*;
    let mut connection = establish_connection();

    let query = query.into_inner();

    let students = student
        .load::<Student>(&mut connection)
        .expect("error loading students");

    let mut res = Vec::new();

    match query.email {
        Some(m) => {
            let filtered: Vec<&Student> = students.iter().filter(|s| s.email == m).collect();
            res = filtered
                .iter()
                .map(|s| StudentSafe::from_student(s))
                .collect();
        }
        None => {
            res = students
                .iter()
                .map(|s| StudentSafe::from_student(s))
                .collect();
        }
    }

    Ok(HttpResponse::Ok().json(res))
}

pub async fn get_student(req: HttpRequest, sid: web::Path<i32>) -> Result<HttpResponse> {
    let authed = auth(&req);
    if authed.is_err() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    use crate::schema::student::dsl::*;
    let mut connection = establish_connection();

    let sid = sid.into_inner();

    let res = student
        .find(sid)
        .first::<Student>(&mut connection)
        .expect("error loading student");

    let res = StudentSafe::from_student(&res);

    Ok(HttpResponse::Ok().json(res))
}

pub async fn get_teachers(req: HttpRequest) -> Result<HttpResponse> {
    let authed = auth(&req);
    if authed.is_err() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    use crate::schema::teacher::dsl::*;
    let mut connection = establish_connection();

    let res = teacher
        .load::<Teacher>(&mut connection)
        .expect("error loading teachers");

    Ok(HttpResponse::Ok().json(res))
}

pub async fn get_teacher(req: HttpRequest, tid: web::Path<i32>) -> Result<HttpResponse> {
    let authed = auth(&req);
    if authed.is_err() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    use crate::schema::teacher::dsl::*;
    let mut connection = establish_connection();

    let tid = tid.into_inner();

    let res = teacher
        .find(tid)
        .first::<Teacher>(&mut connection)
        .expect("error loading teacher");

    Ok(HttpResponse::Ok().json(res))
}
