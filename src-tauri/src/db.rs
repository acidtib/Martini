use diesel::prelude::*;
use tauri::{AppHandle, Manager, path::BaseDirectory};
use chrono::Local;

use crate::models::*;

// Type alias for the database connection
pub type DbConnection = SqliteConnection;

pub fn init(app: &AppHandle) -> DbConnection {
    let database_url = get_db_path(app).unwrap();
    SqliteConnection::establish(&database_url)
        .expect("Failed to create database connection")
}

pub fn save_screenshot(conn: &mut DbConnection, image_data: String) -> Result<i32, diesel::result::Error> {
    let new_screenshot = Screenshot {
        id: None,
        name: "screenshot.jpg".to_string(),
        image: image_data,
        recognized: false,
        ocr: false,
        created_at: Local::now().naive_local(),
    };
    
    diesel::insert_into(screenshots::table)
        .values(&new_screenshot)
        .execute(conn)?;
    
    // Get the last inserted id
    let last_id = diesel::select(diesel::dsl::sql::<diesel::sql_types::Integer>("last_insert_rowid()"))
        .get_result(conn)?;
    
    Ok(last_id)
}

fn get_db_path(app: &AppHandle) -> Result<String, Box<dyn std::error::Error>> {
    Ok(app.path().resolve("app.db", BaseDirectory::AppData)?.to_string_lossy().into_owned())
}