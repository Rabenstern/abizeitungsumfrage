use anyhow::Result;
use serde::Deserialize;
use serde_json::from_reader;

use crate::config::Config;

#[derive(Deserialize)]
pub enum Opt {
    Student,
    Teacher,
}

#[derive(Deserialize)]
pub struct Question {
    pub id: i32,
    pub q: String,
    pub opt1: Option<Opt>,
    pub opt2: Option<Opt>,
    pub opt3: Option<Opt>,
}

#[derive(Deserialize)]
pub struct NewQuestion {
    pub q: String,
    pub opt1: Option<Opt>,
    pub opt2: Option<Opt>,
    pub opt3: Option<Opt>,
}

// for internal (non DB) use
#[derive(Deserialize)]
pub struct Questions {
    pub title: String,
    pub list: Vec<Question>,
}

impl Questions {
    // construct question DB from file
    pub fn construct(conf: &Config) -> Result<Questions> {
        let file = std::fs::File::open(&conf.files.question_file)?;
        let questions: Questions = from_reader(file)?;
        Ok(questions)
    }
}

pub struct Answer {
    pub id: i32,
    pub sid: i32,
    pub qid: i32,
    // either sid or tid
    pub opt1: Option<i32>,
    pub opt2: Option<i32>,
    pub opt3: Option<i32>,
}
