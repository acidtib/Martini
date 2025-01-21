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
                            let start_time = std::time::Instant::now();
                            match screenshot::capture_window(&[".jpg", "notepad", "hunt", "Hunt: Showdown"]) {
                                Ok(image_data) => {
                                    let screenshot_time = start_time.elapsed();
                                    println!("Screenshot captured in {:?}, size: {} bytes", screenshot_time, image_data.len());

                                    let base64_image = STANDARD.encode(&image_data);
                                    let estimated_size_mb = base64_image.len() as f64 / (1024.0 * 1024.0);
                                    println!("Estimated image size: {:.2} MB", estimated_size_mb);

                                    let crop_start = std::time::Instant::now();
                                    match crop::crop_image(handle.clone(), base64_image.clone(), crop::CropRegion::HuntMissionSummary).await {
                                        Ok(cropped_image) => {
                                            let crop_time = crop_start.elapsed();
                                            println!("Image cropped in {:?}", crop_time);

                                            let ocr_start = std::time::Instant::now();
                                            match ocr::perform_ocr(handle.clone(), cropped_image).await {
                                                Ok(text_results) => {
                                                    let ocr_time = ocr_start.elapsed();
                                                    println!("OCR completed in {:?}", ocr_time);

                                                    if text_results.iter().any(|line| line.contains("Mission Summary")) {
                                                        println!("Mission Summary detected, saving screenshot");
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
