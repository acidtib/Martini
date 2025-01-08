use tauri::App;
use tauri::Emitter;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use crate::screenshot;
use base64::Engine;

pub fn register_shortcuts(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(desktop)]
    {
        let ctrl_shift_m = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyM);
       
        app.handle().plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app_handle, shortcut, event| {
                    if shortcut == &ctrl_shift_m && event.state() == ShortcutState::Pressed {
                        println!("Taking screenshot...");
                        match screenshot::capture_window(&["firefox", "hunt", "Hunt: Showdown"]) {
                            Ok(png_data) => {
                                // Convert PNG data to base64
                                let base64_image = base64::engine::general_purpose::STANDARD.encode(&png_data);
                                let data_url = format!("data:image/png;base64,{}", base64_image);
                                
                                // Emit the screenshot data to the frontend
                                if let Err(e) = app_handle.emit("screenshot-taken", data_url) {
                                    println!("Failed to emit screenshot data: {}", e);
                                }
                            },
                            Err(e) => println!("Failed to capture screenshot: {}", e),
                        }
                    }
                })
                .build(),
        )?;
        app.global_shortcut().register(ctrl_shift_m)?;
    }
    Ok(())
}