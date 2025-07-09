use super::schema::files;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = files)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct File {
    pub id: i32,
    pub file_hash: String,
    pub file_name: String,
    pub file_path: String,
    pub size: i32,
    pub private: bool,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = files)]
pub struct NewFile<'a> {
    pub file_hash: &'a str,
    pub file_name: &'a str,
    pub file_path: &'a str,
    pub size: i32,
    pub private: bool,
}
