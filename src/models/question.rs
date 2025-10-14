use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::from_reader;

use crate::{config::Config, entities};

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

#[derive(Serialize, Deserialize)]
pub struct Questions {
    pub title: String,
    pub list: Vec<Question>,
}

/* impl Questions {
    // construct question DB from file
    pub fn construct(conf: &Config) -> Result<Questions> {
        let file = std::fs::File::open(&conf.files.question_file)?;
        let questions: Questions = from_reader(file)?;
        Ok(questions)
    }
} */

pub struct Answer {
    pub id: i32,
    pub sid: i32,
    pub qid: i32,
    // either sid or tid
    pub opt1: Option<i32>,
    pub opt2: Option<i32>,
    pub opt3: Option<i32>,
}
