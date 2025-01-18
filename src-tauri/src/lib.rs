// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::Manager;
use tauri_plugin_sql::{Migration, MigrationKind};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub mod screenshot;
pub mod shortcuts;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migrations = vec![
        Migration {
            version: 1,
            description: "create settings table",
            sql: r#"
                    CREATE TABLE IF NOT EXISTS settings (
                        key TEXT PRIMARY KEY,
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
                        ('installed_on', CURRENT_TIMESTAMP);
                "#,
            kind: MigrationKind::Up,
        },
        Migration {
            version: 3,
            description: "create screenshots table",
            sql: r#"
                    CREATE TABLE IF NOT EXISTS screenshots (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        image BLOB NOT NULL,
                        filename TEXT,
                        mime_type TEXT,
                        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                    );
                "#,
            kind: MigrationKind::Up,
        },
    ];

    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:app.db", migrations)
                .build(),
        )
        .setup(|app| {
            shortcuts::register_shortcuts(app)?;
            Ok(())
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
