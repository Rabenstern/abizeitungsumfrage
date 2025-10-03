use crate::entities;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Teacher {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

impl Teacher {
    pub fn from_model(teacher: &entities::teacher::Model) -> Teacher {
        Teacher {
            id: teacher.id,
            first_name: teacher.first_name.clone(),
            last_name: teacher.last_name.clone(),
        }
    }
}

#[derive(Deserialize)]
pub struct NewTeacher {
    pub first_name: String,
    pub last_name: String,
}
