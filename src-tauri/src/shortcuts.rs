use tauri::App;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

pub fn register_shortcuts(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(desktop)]
    {
        let ctrl_shift_m = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyM);
        
        app.handle().plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |_app, shortcut, event| {
                    if shortcut == &ctrl_shift_m {
                        match event.state() {
                            ShortcutState::Pressed => {
                                println!("Control+Shift+M Pressed!");
                            }
                            ShortcutState::Released => {
                                println!("Control+Shift+M Released!");
                            }
                        }
                    }
                })
                .build(),
        )?;

        app.global_shortcut().register(ctrl_shift_m)?;
    }

    Ok(())
}
