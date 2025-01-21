use diesel::prelude::*;
use tauri::{AppHandle, Manager, path::BaseDirectory};

// Type alias for the database connection
pub type DbConnection = SqliteConnection;

pub fn init(app: &AppHandle) -> DbConnection {
    let database_url = get_db_path(app).unwrap();
    SqliteConnection::establish(&database_url)
        .expect("Failed to create database connection")
}

fn get_db_path(app: &AppHandle) -> Result<String, Box<dyn std::error::Error>> {
    Ok(app.path().resolve("app.db", BaseDirectory::AppData)?.to_string_lossy().into_owned())
}