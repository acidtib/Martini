use crate::crop;
use crate::ocr;
use crate::screenshot;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use serde_json;
use std::error::Error;
use tauri::App;
use tauri::Emitter;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use tokio::runtime::Runtime;

pub fn register_shortcuts(app: &mut App) -> Result<(), Box<dyn Error + Send + Sync>> {
    #[cfg(desktop)]
    {
        let ctrl_shift_m = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyM);
        let rt = Runtime::new()?;

        app.handle().plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app_handle, shortcut, event| {
                    if shortcut == &ctrl_shift_m && event.state() == ShortcutState::Pressed {
                        println!("Taking screenshot...");

                        let handle = app_handle.clone();

                        rt.spawn(async move {
                            match screenshot::capture_window(&[".jpg", "notepad", "hunt", "Hunt: Showdown"]) {
                                Ok(image_data) => {
                                    println!("Screenshot captured, size: {} bytes", image_data.len());

                                    // Convert image data to base64 first
                                    let base64_image = STANDARD.encode(&image_data);
                                    println!("Base64 image size: {} chars", base64_image.len());

                                    // Call the crop_image function with base64 image using the HuntMissionSummary crop region
                                    match crop::crop_image(handle.clone(), base64_image.clone(), crop::CropRegion::HuntMissionSummary).await {
                                        Ok(cropped_image) => {
                                            // Perform OCR on the cropped image
                                            match ocr::perform_ocr(handle.clone(), cropped_image).await {
                                                Ok(text_results) => {
                                                    // Check if any of the lines contain "Mission Summary"
                                                    if text_results.iter().any(|line| line.contains("Mission Summary")) {
                                                        println!("Mission Summary detected, saving screenshot");
                                                        // Tell the main window to save the screenshot
                                                        if let Err(e) = handle.emit("new-screenshot", serde_json::json!({
                                                            "image": base64_image,
                                                            "text": text_results
                                                        })) {
                                                            println!("Failed to emit new-screenshot event: {}", e);
                                                        }
                                                    } else {
                                                        println!("No Mission Summary found in text results");
                                                    }
                                                }
                                                Err(e) => println!("OCR error: {}", e),
                                            }
                                        }
                                        Err(e) => println!("Crop error: {}", e),
                                    }
                                }
                                Err(e) => println!("Screenshot error: {}", e),
                            }
                        });
                    }
                })
                .build(),
        )?;
        app.global_shortcut().register(ctrl_shift_m)?;
    }
    Ok(())
}
