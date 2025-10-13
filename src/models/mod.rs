use serde::{Deserialize, Serialize};

pub mod answer;
pub mod question;
pub mod student;
pub mod teacher;

#[derive(Serialize, Deserialize)]
pub struct PostMessage {
    pub message: String,
}
