use actix_web::{HttpRequest, Result, error::*, http::header::Header};
use actix_web_httpauth::headers::authorization::{Authorization, Basic};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entities::{prelude::*, student};

pub async fn auth(db: &DatabaseConnection, req: &HttpRequest) -> Result<Basic> {
    let auth = Authorization::<Basic>::parse(req)?;

    let auth = auth.as_ref();

    // password = token
    let password = match auth.password() {
        Some(s) => s,
        None => {
            return Err(ErrorBadRequest("failed to auth student, no token provided"));
        }
    };

    let uid = auth.user_id();

    let s = match uid.trim().parse::<i32>() {
        // if uid is int find student with sid == uid
        Ok(i) => Student::find_by_id(i)
            .one(db)
            .await
            .map_err(ErrorInternalServerError)?
            .ok_or(ErrorNotFound(format!(
                "failed to find student via uid: {uid}"
            )))?,

        // if uid isn't int interpret uid as email and find student with email == uid
        Err(_) => Student::find()
            .filter(student::Column::Email.contains(uid))
            .one(db)
            .await
            .map_err(ErrorInternalServerError)?
            .ok_or(ErrorNotFound(format!(
                "failed to find student via email: {uid}"
            )))?,
    };

    if s.token != password {
        return Err(ErrorForbidden("failed to auth student, incorrect token"));
    }

    Ok(auth.clone())
}
