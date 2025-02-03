use crate::crop;
use crate::ocr;
use crate::screenshot;
use crate::AppState;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use std::error::Error;
use tauri::{App, AppHandle, Manager, Emitter, path::BaseDirectory};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use std::sync::atomic::{AtomicBool, Ordering};
use lazy_static::lazy_static;
use std::time::{SystemTime, UNIX_EPOCH};
use image::ImageError;
use crate::models::settings::settings::dsl::*;
use diesel::prelude::*;

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

            // Save the screenshot as JPEG
            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
            let debug_path = app_handle.path().resolve("debug_images", BaseDirectory::AppData)?;
            std::fs::create_dir_all(&debug_path).map_err(|e| ImageError::IoError(e))?;
            let filename = format!("{}/screenshot_{}.jpg", debug_path.to_str().unwrap(), timestamp);
            std::fs::write(&filename, &image_data)?;

            Ok(Some(base64_image))
        }
        Err(e) => {
            println!("Error capturing screenshot: {:?}", e);
            Ok(None)
        }
    }
}

async fn crop_image(app_handle: &AppHandle, base64_image: &str, region: crop::CropRegion) -> Result<String, Box<dyn Error + Send + Sync>> {
    let _ = app_handle.emit("screenshot-status", "cropping");
    println!("Base64 image length: {}", base64_image.len());
    println!("Base64 image (first 100 chars): {}", &base64_image[..100.min(base64_image.len())]);
    let crop_start = std::time::Instant::now();
    match crop::crop_image(app_handle.clone(), base64_image.to_string(), region).await {
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

async fn perform_ocr(app_handle: &AppHandle, base64_image: &str) -> Result<Option<i32>, Box<dyn Error + Send + Sync>> {
    let _ = app_handle.emit("screenshot-status", "recognizing");
    
    // First, check if it's a mission summary screen
    let mission_summary_crop = crop_image(app_handle, base64_image, crop::CropRegion::MissionSummary).await?;
    let mission_summary_text = ocr::perform_ocr(app_handle.clone(), mission_summary_crop).await?;
    
    let has_mission_summary = mission_summary_text.iter()
        .any(|line| line.to_lowercase().contains("mission summary"));
    
    if has_mission_summary {
        // If it is a mission summary, check the first summary region for mission type
        let summary_first_crop = crop_image(app_handle, base64_image, crop::CropRegion::SummaryFirst).await?;
        let summary_first_text = ocr::perform_ocr(app_handle.clone(), summary_first_crop).await?;
        
        let has_bounty_mission = summary_first_text.iter()
            .any(|line| line.to_lowercase().contains("bounty collected"));
        let has_soul_survival = summary_first_text.iter()
            .any(|line| line.to_lowercase().contains("rifts closed"));
        
        // Determine mission type
        let mission_type = if has_bounty_mission {
            "bounty"
        } else if has_soul_survival {
            "soul_survival"
        } else {
            "unknown"
        };
        
        println!("Detected mission type: {}", mission_type);
        
        // Update database if a valid mission type is detected
        if mission_type != "unknown" {
            if let Some(db) = app_handle.state::<AppState>().inner().db.as_ref() {
                if let Ok(mut conn) = db.lock() {
                    match crate::db::save_screenshot(&mut conn, base64_image.to_string(), mission_type.to_string()) {
                        Ok(screenshot_id) => {
                            println!("Screenshot saved to database with id: {}", screenshot_id);
                            
                            use diesel::prelude::*;
                            use crate::models::screenshots::dsl::*;
                            
                            // Update the recognized field for the specific screenshot
                            diesel::update(screenshots.filter(id.eq(screenshot_id)))
                                .set(recognized.eq(true))
                                .execute(&mut *conn)
                                .unwrap_or_else(|e| {
                                    println!("Error updating screenshot recognized status: {:?}", e);
                                    0
                                });

                            let _ = app_handle.emit("screenshot-status", "detected");
                            let _ = app_handle.emit("open-screenshot-viewer", ());
                            println!("Mission Summary detected");
                            return Ok(Some(screenshot_id));
                        }
                        Err(e) => {
                            println!("Error saving screenshot: {:?}", e);
                            return Ok(None);
                        }
                    }
                }
            }
        }
    }
    
    let _ = app_handle.emit("screenshot-status", "not-detected");
    println!("No valid mission summary detected");
    Ok(None)
}

fn get_shortcut(app: &App) -> Result<String, Box<dyn Error + Send + Sync>> {
    if let Some(db) = app.state::<AppState>().inner().db.as_ref() {
        if let Ok(mut conn) = db.lock() {
            let shortcut_value: String = settings
                .filter(key.eq("shortcut"))
                .select(value)
                .first(&mut *conn)?;
            return Ok(shortcut_value);
        }
    }
    Ok("Ctrl+Shift+M".to_string()) // Default fallback if database query fails
}

fn format_key_for_code(input_key: &str) -> String {
    let key_str = input_key.trim().to_uppercase();
    // Single letters need to be prefixed with "Key"
    if key_str.len() == 1 && key_str.chars().next().unwrap().is_ascii_alphabetic() {
        format!("Key{}", key_str)
    } else {
        key_str
    }
}

pub fn register_shortcuts(app: &mut App) -> Result<(), Box<dyn Error + Send + Sync>> {
    #[cfg(desktop)]
    {
        let shortcut_str = get_shortcut(app)?;
        let parts: Vec<&str> = shortcut_str.split('+').collect();
        
        let mut modifiers = Modifiers::empty();
        let mut key_part = None;
        
        // Process each part of the shortcut string
        for part in parts.iter() {
            let part = part.trim().to_uppercase();
            match part.as_str() {
                "CTRL" | "CONTROL" => modifiers |= Modifiers::CONTROL,
                "SHIFT" => modifiers |= Modifiers::SHIFT,
                "ALT" => modifiers |= Modifiers::ALT,
                "SUPER" | "WIN" | "CMD" | "COMMAND" => modifiers |= Modifiers::SUPER,
                input_key => key_part = Some(input_key.to_string()),
            }
        }
        
        // Parse the key using Code's FromStr implementation
        let code = match key_part {
            Some(input_key) => {
                let formatted_key = format_key_for_code(&input_key);
                formatted_key.parse::<Code>()
                    .map_err(|_| format!("Invalid key in shortcut: {}", input_key))?
            }
            None => return Err("No key specified in shortcut".into()),
        };

        let shortcut = Shortcut::new(Some(modifiers), code);
        let app_handle = app.handle().clone();
        let app_handle_clone = app_handle.clone();

        app_handle.plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |_shortcut_handle, shortcut_pressed, event| {
                    if shortcut_pressed == &shortcut && event.state() == ShortcutState::Pressed {
                        // Check if we're already processing a screenshot
                        if IS_PROCESSING.load(Ordering::SeqCst) {
                            println!("Screenshot processing in progress, please wait...");
                            return;
                        }

                        // Set the processing flag
                        IS_PROCESSING.store(true, Ordering::SeqCst);
                        println!("Taking screenshot...");

                        let app_handle = app_handle_clone.clone();

                        // let _ = app_handle.emit("close-screenshot-viewer", ());
                        let _ = app_handle.emit("screenshot-status", "capturing");

                        tauri::async_runtime::spawn(async move {
                            let _result = async {
                                if let Ok(Some(base64_image)) = capture_screenshot(&app_handle).await {
                                    // let _ = app_handle.emit("open-screenshot-viewer", ());

                                    match crop_image(&app_handle, &base64_image, crop::CropRegion::MissionSummary).await {
                                        Ok(_) => {
                                            let _ = perform_ocr(&app_handle, &base64_image).await;

                                            // let _ = app_handle.emit("open-screenshot-viewer", ());
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
        app.global_shortcut().register(shortcut)?;
    }
    Ok(())
}
