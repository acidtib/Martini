use tauri::App;
use tauri::Manager;
use tauri::Emitter;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use serde_json;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use crate::screenshot;

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
                            Ok(image_data) => {
                                println!("Screenshot captured, size: {} bytes", image_data.len());
                                
                                // Convert image data to base64 first
                                let base64_image = STANDARD.encode(&image_data);
                                println!("Base64 image size: {} chars", base64_image.len());

                                // Tell the main window to create/show the viewer window
                                if let Err(e) = app_handle.emit("open-viewer", ()) {
                                    println!("Failed to emit open-viewer event: {}", e);
                                    return;
                                }

                                // Give the window more time to initialize
                                std::thread::sleep(std::time::Duration::from_millis(1000));

                                // Send the screenshot data to the viewer
                                if let Err(e) = app_handle.emit("screenshot-data", serde_json::json!({
                                    "base64Image": base64_image
                                })) {
                                    println!("Failed to emit screenshot data: {}", e);
                                } else {
                                    println!("Screenshot data sent successfully");
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