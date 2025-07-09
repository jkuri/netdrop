pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::models::{NewFile, File};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_file(conn: &mut SqliteConnection, new_file: NewFile<'_>) -> File {
    use crate::schema::files;

    diesel::insert_into(files::table)
        .values(&new_file)
        .returning(File::as_returning())
        .get_result(conn)
        .expect("Error saving file")
}

pub fn get_file_by_hash(conn: &mut SqliteConnection, hash: &str) -> Option<File> {
    use crate::schema::files::dsl::*;

    files
        .filter(file_hash.eq(hash))
        .first::<File>(conn)
        .ok()
}
