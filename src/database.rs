use anyhow::Result;
use sea_orm::{DatabaseConnection, EntityTrait, Set};

use crate::{
    config::Config,
    entities,
    models::{
        question::NewQuestion,
        question::Opt::{Student, Teacher},
        student::{NewStudent, StudentCSV},
        teacher::NewTeacher,
    },
};

/// load students into DB from file
pub async fn load_students(db: &DatabaseConnection, cfg: &Config) -> Result<()> {
    if let Some(_) = entities::student::Entity::find().one(db).await? {
        log::info!("DB already has students");
        return Ok(());
    }

    let mut rdr = csv::Reader::from_path(&cfg.files.students_file)?;

    let mut new_students = Vec::new();

    for result in rdr.deserialize() {
        let result: StudentCSV = result?;
        let s: NewStudent = NewStudent::from_student_csv(&result)?;

        let new_student = entities::student::ActiveModel {
            email: Set(s.email),
            token: Set(s.token),
            first_name: Set(s.first_name),
            last_name: Set(s.last_name),
            ..Default::default()
        };

        new_students.push(new_student);
    }

    entities::student::Entity::insert_many(new_students)
        .exec(db)
        .await?;

    Ok(())
}

/// load teachers into DB from file
pub async fn load_teachers(db: &DatabaseConnection, cfg: &Config) -> Result<()> {
    if let Some(_) = entities::teacher::Entity::find().one(db).await? {
        log::info!("DB already has teacher");
        return Ok(());
    }

    let mut rdr = csv::Reader::from_path(&cfg.files.teachers_file)?;

    let mut new_teachers = Vec::new();

    for result in rdr.deserialize() {
        let t: NewTeacher = result?;

        let new_teacher = entities::teacher::ActiveModel {
            first_name: Set(t.first_name),
            last_name: Set(t.last_name),
            ..Default::default()
        };

        new_teachers.push(new_teacher);
    }

    entities::teacher::Entity::insert_many(new_teachers)
        .exec(db)
        .await?;

    Ok(())
}

/// load questions into DB from file
pub async fn load_questions(db: &DatabaseConnection, cfg: &Config) -> Result<()> {
    if let Some(_) = entities::question::Entity::find().one(db).await? {
        log::info!("DB already has questions");
        return Ok(());
    }

    let mut rdr = csv::Reader::from_path(&cfg.files.question_file)?;

    let mut new_questions = Vec::new();

    for result in rdr.deserialize() {
        let q: NewQuestion = result?;

        let new_question = entities::question::ActiveModel {
            q: Set(Some(q.q)),
            opt1: Set(q.opt1.map(|x| match x {
                Student => String::from("Student"),
                Teacher => String::from("Teacher"),
            })),
            opt2: Set(q.opt2.map(|x| match x {
                Student => String::from("Student"),
                Teacher => String::from("Teacher"),
            })),
            opt3: Set(q.opt3.map(|x| match x {
                Student => String::from("Student"),
                Teacher => String::from("Teacher"),
            })),
            ..Default::default()
        };

        new_questions.push(new_question);
    }

    entities::question::Entity::insert_many(new_questions)
        .exec(db)
        .await?;

    Ok(())
}
