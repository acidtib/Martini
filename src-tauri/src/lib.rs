// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::Manager;
use tauri_plugin_sql::{Migration, MigrationKind};
use anyhow::anyhow;
use std::sync::Mutex;

#[derive(Default)]
pub struct AppState {
    pub db: Option<Mutex<db::DbConnection>>,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub mod db;
pub mod models;
pub mod screenshot;
pub mod shortcuts;
pub mod crop;
pub mod ocr;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migrations = vec![
        Migration {
            version: 1,
            description: "create settings table",
            sql: r#"
                    CREATE TABLE IF NOT EXISTS settings (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        key TEXT NOT NULL,
                        value TEXT NOT NULL
                    );
                "#,
            kind: MigrationKind::Up,
        },
        Migration {
            version: 2,
            description: " Insert initial settings",
            sql: r#"
                    INSERT INTO settings (key, value) VALUES
                        ('bootstrapped', 'false'),
                        ('installed_on', CURRENT_TIMESTAMP),
                        ('system_os', '-'),
                        ('system_cpu', '-'),
                        ('system_memory', '-');
                "#,
            kind: MigrationKind::Up,
        },
        Migration {
            version: 3,
            description: "create screenshots table",
            sql: r#"
                    CREATE TABLE IF NOT EXISTS screenshots (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        name TEXT NOT NULL,
                        image TEXT NOT NULL,
                        recognized BOOLEAN DEFAULT 0,
                        ocr BOOLEAN DEFAULT 0,
                        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                    );
                "#,
            kind: MigrationKind::Up,
        },
    ];

    tauri::Builder::default()
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api: _, .. } => { 
                // if the main window is closing also close the other windows that couls be opened
                if window.label() == "main" {
                    let windows = window.windows();
                    for (label, win) in windows {
                        if label != "main" {
                            if let Err(e) = win.close() {
                                eprintln!("Failed to close window {}: {}", label, e);
                            }
                        }
                    }
                }
            } _ => {} 
        })
        .plugin(tauri_plugin_system_info::init())
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:app.db", migrations)
                .build(),
        )
        .setup(|app| {
            tauri::async_runtime::block_on(async move {
                let app_handle = app.handle();
                let conn = db::init(&app_handle);
                
                // Store the database connection in the app state
                app.manage(AppState {
                    db: Some(Mutex::new(conn)),
                });

                // Initialize system info
                if let Some(db_mutex) = app.state::<AppState>().db.as_ref() {
                    if let Ok(mut conn) = db_mutex.lock() {
                        if let Err(e) = db::save_system_info(&mut conn).await {
                            eprintln!("Failed to save system info: {}", e);
                        }
                    }
                }

                shortcuts::register_shortcuts(app).map_err(|e| anyhow!("Failed to register shortcuts: {}", e))?;
                Ok(())
            })
        })
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_single_instance::init(|app, argv, _cwd| {
            // handle second instance here
            println!("new app instance with args: {argv:?}");
            // the deep link event was already triggered at this point

            // bring the main window to the foreground
            app.get_webview_window("main")
                .expect("no main window")
                .set_focus()
                .expect("failed to focus window");
        }))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
