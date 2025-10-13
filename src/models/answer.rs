use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::entities;

#[derive(Serialize, Deserialize)]
pub struct Answer {
    pub id: i32,
    pub sid: i32,
    pub qid: i32,
    pub opt1: Option<i32>,
    pub opt2: Option<i32>,
    pub opt3: Option<i32>,
}

impl Answer {
    pub fn from_model(answer: &entities::answer::Model) -> Answer {
        Answer {
            id: answer.id,
            sid: answer.sid,
            qid: answer.qid,
            opt1: answer.opt1,
            opt2: answer.opt2,
            opt3: answer.opt3,
        }
    }
}

#[derive(Deserialize)]
pub struct NewAnswer {
    pub sid: i32,
    pub qid: i32,
    pub opt1: Option<i32>,
    pub opt2: Option<i32>,
    pub opt3: Option<i32>,
}
