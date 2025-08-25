use anyhow::Result;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{student, teacher};

#[derive(Serialize, Deserialize)]
pub struct PostMessage {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct StudentsQuery {
    pub email: Option<String>,
}

#[derive(Queryable, Serialize)]
pub struct Student {
    pub id: i32,
    pub email: String,
    pub token: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct StudentSafe {
    pub id: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

impl StudentSafe {
    pub fn from_student(student: &Student) -> StudentSafe {
        StudentSafe {
            id: student.id,
            email: student.email.clone(),
            first_name: student.first_name.clone(),
            last_name: student.last_name.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct StudentCSV {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = student)]
pub struct NewStudent {
    pub email: String,
    pub token: String,
    pub first_name: String,
    pub last_name: String,
}

impl NewStudent {
    pub fn from_student_csv(student: &StudentCSV) -> Result<NewStudent> {
        let salt = std::env::var("TOKEN_SALT")?;
        let str = format!("{}{}", student.email, salt);
        let token: String = sha256::digest(str);

        Ok(NewStudent {
            email: student.email.clone(),
            token,
            first_name: student.first_name.clone(),
            last_name: student.last_name.clone(),
        })
    }
}

#[derive(Queryable, Serialize)]
pub struct Teacher {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = teacher)]
pub struct NewTeacher {
    pub first_name: String,
    pub last_name: String,
}
