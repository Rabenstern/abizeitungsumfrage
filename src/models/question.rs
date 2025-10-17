use serde::{Deserialize, Serialize};

use crate::entities;

#[derive(Serialize, Deserialize)]
pub enum Opt {
    Student,
    Teacher,
}

#[derive(Serialize, Deserialize)]
pub struct Question {
    pub id: i32,
    pub q: String,
    pub opt1: Option<Opt>,
    pub opt2: Option<Opt>,
    pub opt3: Option<Opt>,
}

impl Question {
    pub fn from_model(question: &entities::question::Model) -> Question {
        let opt1: Option<Opt> = match &question.opt1 {
            Some(s) => match s.as_str() {
                "Student" => Some(Opt::Student),
                "Teacher" => Some(Opt::Teacher),
                _ => None,
            },
            None => None,
        };

        let opt2: Option<Opt> = match &question.opt2 {
            Some(s) => match s.as_str() {
                "Student" => Some(Opt::Student),
                "Teacher" => Some(Opt::Teacher),
                _ => None,
            },
            None => None,
        };

        let opt3: Option<Opt> = match &question.opt3 {
            Some(s) => match s.as_str() {
                "Student" => Some(Opt::Student),
                "Teacher" => Some(Opt::Teacher),
                _ => None,
            },
            None => None,
        };

        Question {
            id: question.id,
            q: question.q.clone(),
            opt1,
            opt2,
            opt3,
        }
    }
}

#[derive(Deserialize)]
pub struct NewQuestion {
    pub q: String,
    pub opt1: Option<Opt>,
    pub opt2: Option<Opt>,
    pub opt3: Option<Opt>,
}

/* #[derive(Serialize, Deserialize)]
pub struct Questions {
    pub title: String,
    pub list: Vec<Question>,
} */
