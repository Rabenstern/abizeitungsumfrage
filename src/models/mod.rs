use serde::{Deserialize, Serialize};

pub mod question;
pub mod student;
pub mod teacher;

#[derive(Serialize, Deserialize)]
pub struct PostMessage {
    pub message: String,
}
