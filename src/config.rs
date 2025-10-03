#![allow(dead_code)]

use anyhow::Result;
use serde::Deserialize;
use std::{fs::File, io::Read};
use toml::from_str;

use crate::CONFIG_FILE;

/// ```toml
/// title = "title of questionaire"
///
/// \[files\]
/// students_file = "students.csv"
/// teachers_file = "teachers.csv"
/// question_file = "questions.json"
///
/// \[db\]
/// database_url = "sqlite://./database.db?mode=rwc"
/// ```
#[derive(Deserialize, Clone)]
pub struct Config {
    pub title: String,
    pub files: Files,
    pub db: Db,
}

impl Config {
    /// parse config from file
    pub fn new() -> Result<Config> {
        let mut file = File::open(CONFIG_FILE)?;
        let mut string = String::new();

        file.read_to_string(&mut string)?;
        let config = from_str(&string)?;

        Ok(config)
    }
}

// pub students_file: "students.csv",
// pub teachers_file: "teachers.csv",
// pub question_file: "questions.json",
#[derive(Deserialize, Clone)]
pub struct Files {
    pub students_file: String,
    pub teachers_file: String,
    pub question_file: String,
}

// pub database_url: "sqlite://./database.db?mode=rwc",
#[derive(Deserialize, Clone)]
pub struct Db {
    pub database_url: String,
}
