use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::entities;

#[derive(Deserialize)]
pub struct StudentsQuery {
    pub email: Option<String>,
}

#[derive(Serialize, Clone)]
pub struct Student {
    pub id: i32,
    pub email: String,
    pub token: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Deserialize, Serialize)]
pub struct StudentSafe {
    pub id: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

impl StudentSafe {
    pub fn from_model(student: &entities::student::Model) -> StudentSafe {
        StudentSafe {
            id: student.id,
            email: student.email.clone(),
            first_name: student.first_name.clone(),
            last_name: student.last_name.clone(),
        }
    }
}

#[derive(Deserialize)]
pub struct StudentCSV {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Deserialize)]
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
