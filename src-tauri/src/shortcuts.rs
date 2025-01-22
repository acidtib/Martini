use crate::crop;
use crate::ocr;
use crate::screenshot;
use crate::AppState;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use std::error::Error;
use tauri::App;
use tauri::AppHandle;
use tauri::Manager;
use tauri::Emitter;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use std::sync::atomic::{AtomicBool, Ordering};
use lazy_static::lazy_static;

lazy_static! {
    static ref IS_PROCESSING: AtomicBool = AtomicBool::new(false);
}

async fn capture_screenshot(app_handle: &AppHandle) -> Result<Option<String>, Box<dyn Error + Send + Sync>> {
    let start_time = std::time::Instant::now();
    match screenshot::capture_window(&[".jpg", "notepad", "hunt", "Hunt: Showdown"]) {
        Ok(image_data) => {
            let screenshot_time = start_time.elapsed();
            println!("Screenshot captured in {:?}, size: {} bytes", screenshot_time, image_data.len());

            let base64_image = STANDARD.encode(&image_data);
            let estimated_size_mb = base64_image.len() as f64 / (1024.0 * 1024.0);
            println!("Estimated image size: {:.2} MB", estimated_size_mb);

            let state = app_handle.state::<AppState>();
            if let Some(db) = state.inner().db.as_ref() {
                if let Ok(mut conn) = db.lock() {
                    match crate::db::save_screenshot(&mut conn, base64_image.clone()) {
                        Ok(id) => {
                            println!("Screenshot saved to database with id: {}", id);
                            return Ok(Some(base64_image));
                        }
                        Err(e) => println!("Error saving screenshot: {:?}", e),
                    }
                } else {
                    println!("Could not lock database connection");
                }
            }
            Ok(Some(base64_image))
        }
        Err(e) => {
            println!("Error capturing screenshot: {:?}", e);
            Ok(None)
        }
    }
}

async fn crop_image(app_handle: &AppHandle, base64_image: String) -> Result<String, Box<dyn Error + Send + Sync>> {
    println!("Base64 image length: {}", base64_image.len());
    println!("Base64 image (first 100 chars): {}", &base64_image[..100.min(base64_image.len())]);
    let crop_start = std::time::Instant::now();
    match crop::crop_image(app_handle.clone(), base64_image, crop::CropRegion::HuntMissionSummary).await {
        Ok(cropped_image) => {
            let crop_time = crop_start.elapsed();
            println!("Image cropped in {:?}", crop_time);
            Ok(cropped_image)
        }
        Err(e) => {
            println!("Error cropping image: {:?}", e);
            Err(e.into())
        }
    }
}

async fn perform_ocr(app_handle: &AppHandle, cropped_image: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    let ocr_start = std::time::Instant::now();
    match ocr::perform_ocr(app_handle.clone(), cropped_image).await {
        Ok(text_results) => {
            let ocr_time = ocr_start.elapsed();
            println!("OCR completed in {:?}", ocr_time);

            if text_results.iter().any(|line| line.contains("Mission Summary")) {
                println!("Mission Summary detected");
            } else {
                println!("No Mission Summary detected");
            }
            Ok(())
        }
        Err(e) => {
            println!("Error performing OCR: {:?}", e);
            Err(e.into())
        }
    }
}

pub fn register_shortcuts(app: &mut App) -> Result<(), Box<dyn Error + Send + Sync>> {
    #[cfg(desktop)]
    {
        let ctrl_shift_m = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyM);

        let app_handle = app.handle().clone();
        let app_handle_clone = app_handle.clone();

        app_handle.plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |_shortcut_handle, shortcut, event| {
                    if shortcut == &ctrl_shift_m && event.state() == ShortcutState::Pressed {
                        // Check if we're already processing a screenshot
                        if IS_PROCESSING.load(Ordering::SeqCst) {
                            println!("Screenshot processing in progress, please wait...");
                            return;
                        }

                        // Set the processing flag
                        IS_PROCESSING.store(true, Ordering::SeqCst);
                        println!("Taking screenshot...");

                        let app_handle = app_handle_clone.clone();
                        tauri::async_runtime::spawn(async move {
                            let _result = async {
                                if let Ok(Some(base64_image)) = capture_screenshot(&app_handle).await {
                                    let _ = app_handle.emit("open-screenshot-viewer", ());

                                    match crop_image(&app_handle, base64_image).await {
                                        Ok(cropped_image) => {
                                            let _ = perform_ocr(&app_handle, cropped_image).await;
                                        }
                                        Err(e) => println!("Error in cropping: {:?}", e),
                                    }
                                }
                            }.await;

                            // Reset the processing flag when we're done
                            IS_PROCESSING.store(false, Ordering::SeqCst);
                        });
                    }
                })
                .build(),
        )?;
        app.global_shortcut().register(ctrl_shift_m)?;
    }
    Ok(())
}
