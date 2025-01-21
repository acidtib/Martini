use crate::crop;
use crate::ocr;
use crate::screenshot;
use crate::AppState;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use std::error::Error;
use tauri::App;
use tauri::Manager;
use tauri::Emitter;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use std::sync::Arc;

pub fn register_shortcuts(app: &mut App) -> Result<(), Box<dyn Error + Send + Sync>> {
    #[cfg(desktop)]
    {
        let ctrl_shift_m = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyM);
        
        let app_handle = app.handle().clone();
        
        app.handle().plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |_shortcut_handle, shortcut, event| {
                    if shortcut == &ctrl_shift_m && event.state() == ShortcutState::Pressed {
                        println!("Taking screenshot...");

                        let state = app_handle.state::<AppState>();
                        let handle = app_handle.clone();
                        let state = state;

                        tokio::spawn(async move {
                            let start_time = std::time::Instant::now();
                            match screenshot::capture_window(&[".jpg", "notepad", "hunt", "Hunt: Showdown"]) {
                                Ok(image_data) => {
                                    let screenshot_time = start_time.elapsed();
                                    println!("Screenshot captured in {:?}, size: {} bytes", screenshot_time, image_data.len());  

                                    // Convert to base64 for the crop function
                                    let base64_image = STANDARD.encode(&image_data);
                                    let estimated_size_mb = base64_image.len() as f64 / (1024.0 * 1024.0);
                                    println!("Estimated image size: {:.2} MB", estimated_size_mb);                                  

                                    // Save screenshot to database first
                                    let screenshot_id = if let Some(db) = &state.db {
                                        if let Ok(mut conn) = db.lock() {
                                            match crate::db::save_screenshot(&mut conn, base64_image.clone()) {
                                                Ok(id) => {
                                                    println!("Screenshot saved to database with id: {}", id);
                                                    Some(id)
                                                }
                                                Err(e) => {
                                                    println!("Error saving screenshot: {:?}", e);
                                                    None
                                                }
                                            }
                                        } else {
                                            println!("Could not lock database connection");
                                            None
                                        }
                                    } else {
                                        None
                                    };

                                    tokio::spawn(async move {
                                        // open the screenshot viewer window
                                        let _ = handle.emit("open-screenshot-viewer", ());

                                        // Crop the screenshot for Hunt: Showdown mission summary
                                        let crop_start = std::time::Instant::now();
                                        match crop::crop_image(handle.clone(), base64_image, crop::CropRegion::HuntMissionSummary).await {
                                            Ok(cropped_image) => {
                                                let crop_time = crop_start.elapsed();
                                                println!("Image cropped in {:?}", crop_time);
                                                println!("Screenshot cropped successfully");

                                                let ocr_start = std::time::Instant::now();
                                                match ocr::perform_ocr(handle.clone(), cropped_image).await {
                                                    Ok(text_results) => {
                                                        let ocr_time = ocr_start.elapsed();
                                                        println!("OCR completed in {:?}", ocr_time);

                                                        if text_results.iter().any(|line| line.contains("Mission Summary")) {
                                                            println!("Mission Summary detected");
                                                            
                                                        } else {
                                                            println!("No Mission Summary found in text results");
                                                        }
                                                    }
                                                    Err(e) => println!("OCR error: {}", e),
                                                }
                                            }
                                            Err(e) => {
                                                println!("Error cropping screenshot: {}", e);
                                            }
                                        }
                                    });
                                }
                                Err(e) => println!("Screenshot error: {}", e),
                            }
                            println!("Total processing time: {:?}", start_time.elapsed());
                        });
                    }
                })
                .build(),
        )?;
        app.global_shortcut().register(ctrl_shift_m)?;
    }
    Ok(())
}
