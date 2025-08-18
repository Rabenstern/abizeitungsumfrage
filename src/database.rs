use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to the database: {}", database_url))
}

pub fn load_data() -> Result<(), Box<dyn std::error::Error>> {
    let connection = establish_connection();
    // check if students and or teachers DB has entries
    // if not, read CSV and populate DB
    Ok(())
}
