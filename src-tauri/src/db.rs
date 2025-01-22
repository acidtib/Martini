use diesel::prelude::*;
use tauri::{AppHandle, Manager, path::BaseDirectory};
use chrono::Local;

use crate::models::{Setting, screenshots, Screenshot};

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

pub async fn save_system_info(conn: &mut DbConnection) -> Result<(), Box<dyn std::error::Error>> {
    use tauri_plugin_system_info::utils::SysInfoState;
    use crate::models::settings::settings::dsl::*;
    use diesel::dsl::exists;
    use diesel::select;
    
    // Initialize system info state
    let state = SysInfoState::default();
    let mut sysinfo = state.sysinfo.lock().unwrap();

    // Get system information
    let cpu_count = sysinfo.cpu_count();
    let os_name = sysinfo.name().unwrap_or_default();
    let os_version = sysinfo.os_version().unwrap_or_default();
    let memory = sysinfo.total_memory();

    // Format memory size in GB
    let total_memory_gb = memory as f64 / (1024.0 * 1024.0 * 1024.0);
    let formatted_memory = format!("{:.2} GB", total_memory_gb);

    // Create system info map
    let system_info = vec![
        ("system_os", format!("{} {}", os_name, os_version)),
        ("system_cpu", format!("{} Cores", cpu_count)),
        ("system_memory", formatted_memory),
    ];

    // Update or insert each system info setting
    for (setting_key, setting_value) in system_info {
        let exists = select(exists(settings.filter(key.eq(setting_key.to_string())))).get_result(conn)?;
        
        if exists {
            diesel::update(settings.filter(key.eq(setting_key.to_string())))
                .set(value.eq(setting_value))
                .execute(conn)?;
        } else {
            diesel::insert_into(settings)
                .values(&Setting {
                    id: None,
                    key: setting_key.to_string(),
                    value: setting_value,
                })
                .execute(conn)?;
        }
    }

    Ok(())
}

fn get_db_path(app: &AppHandle) -> Result<String, Box<dyn std::error::Error>> {
    Ok(app.path().resolve("app.db", BaseDirectory::AppData)?.to_string_lossy().into_owned())
}