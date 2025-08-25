use actix_web::{HttpRequest, http::header::Header};
use actix_web_httpauth::headers::authorization::{Authorization, Basic};
use anyhow::{Result, anyhow};
use diesel::prelude::*;

use crate::database::establish_connection;
use crate::models::Student;

pub fn auth(req: &HttpRequest) -> Result<Basic> {
    use crate::schema::student::dsl::*;

    let mut connection = establish_connection();

    let auth = Authorization::<Basic>::parse(req)?;

    let auth = auth.as_ref();

    // password = token
    let password = match auth.password() {
        Some(s) => s,
        None => return Err(anyhow!("failed to auth student, no token provided")),
    };

    let uid = auth.user_id();

    let s = match uid.trim().parse::<i32>() {
        // if uid is int find student with sid == uid
        Ok(i) => student.find(i).first::<Student>(&mut connection)?,
        // if uid isn't int interpret uid as email and find student with email == uid
        Err(_) => student
            .load::<Student>(&mut connection)?
            .iter()
            .find(|x| x.email == uid)
            .ok_or(anyhow!("failed to find student via email: {uid}"))?
            .clone(),
    };

    if s.token != password {
        return Err(anyhow!("failed to auth student, incorrect token",));
    }

    Ok(auth.clone())
}
