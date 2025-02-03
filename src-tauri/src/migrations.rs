use tauri_plugin_sql::{Migration, MigrationKind};

pub fn get_migrations() -> Vec<Migration> {
    vec![
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
            description: "Insert initial settings",
            sql: r#"
                    INSERT INTO settings (key, value) VALUES
                        ('bootstrapped', 'false'),
                        ('installed_on', CURRENT_TIMESTAMP),
                        ('shortcut', 'Ctrl+Shift+M'),
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
                        mission_type TEXT NOT NULL,
                        name TEXT NOT NULL,
                        image TEXT NOT NULL,
                        recognized BOOLEAN DEFAULT 0,
                        ocr BOOLEAN DEFAULT 0,
                        summary_first TEXT,
                        summary_second TEXT,
                        summary_third TEXT,
                        summary_fourth TEXT,
                        summary_username TEXT,
                        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                    );
                "#,
            kind: MigrationKind::Up,
        },
    ]
}
