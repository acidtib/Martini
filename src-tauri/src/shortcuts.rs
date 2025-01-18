use crate::screenshot;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use serde_json;
use tauri::App;
use tauri::Emitter;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

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

                                // Tell the main window to save the screenshot
                                if let Err(e) = app_handle.emit("new-screenshot", serde_json::json!({
                                    "image": base64_image.clone()
                                })) {
                                    println!("Failed to emit new-screenshot event: {}", e);
                                    return;
                                }

                                // // Tell the main window to create/show the viewer window
                                // if let Err(e) = app_handle.emit("open-viewer", serde_json::json!({
                                //     "event": "open-viewer",
                                //     "data": {
                                //         "image": base64_image
                                //     }
                                // })) {
                                //     println!("Failed to emit open-viewer event: {}", e);
                                //     return;
                                // }
                            }
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
