use anyhow::Result;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use crate::models::{NewStudent, NewTeacher, StudentCSV};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to the database: {}", database_url))
}

pub fn load_data() -> Result<()> {
    use crate::schema::student::dsl::*;
    use crate::schema::teacher::dsl::*;

    let mut connection = establish_connection();

    // check if students and or teachers DB has entries
    let count_s: i64 = student
        .count()
        .get_result(&mut connection)
        .expect("failed to count students");

    if count_s == 0 {
        // if not, read CSV and populate DB
        let mut rdr =
            csv::Reader::from_path("students.csv").expect("failed to open file students.csv");

        for result in rdr.deserialize() {
            let result: StudentCSV = result?;

            let s = NewStudent::from_student_csv(&result)?;

            diesel::insert_into(student)
                .values(s)
                .execute(&mut connection)?;
        }
    }

    let count_t: i64 = teacher
        .count()
        .get_result(&mut connection)
        .expect("failed to count teachers");

    if count_t == 0 {
        // if not, read CSV and populate DB
        let mut rdr =
            csv::Reader::from_path("teachers.csv").expect("failed to open file teachers.csv");

        for result in rdr.deserialize() {
            let result: NewTeacher = result?;

            let t = NewTeacher {
                first_name: result.first_name,
                last_name: result.last_name,
            };

            diesel::insert_into(teacher)
                .values(t)
                .execute(&mut connection)?;
        }
    }

    Ok(())
}
