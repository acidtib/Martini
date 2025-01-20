use image::{GenericImageView, ImageError};
use image::ImageFormat;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use std::io::Cursor;
use tauri::{AppHandle, Runtime, Emitter};

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum CropRegion {
    HuntMissionSummary,
    HuntStatBountyToken
}

#[derive(Debug, Clone, Copy)]
struct CropConfig {
    x: u32,
    y: u32,
    width: u32,
    height: u32
}

fn get_crop_config(region: CropRegion) -> CropConfig {
    match region {
        CropRegion::HuntMissionSummary => CropConfig {
            x: 130,
            y: 95,
            width: 400,
            height: 95,
        },

        CropRegion::HuntStatBountyToken => CropConfig {
            x: 616,
            y: 300,
            width: 356,
            height: 90,
        },
    }
}

fn calculate_proportional_dimensions(
    original_width: u32,
    original_height: u32,
    config: &CropConfig,
) -> (u32, u32, u32, u32) {
    let width_ratio = original_width as f32 / 1920.0;
    let height_ratio = original_height as f32 / 1080.0;

    let new_x = (config.x as f32 * width_ratio).round() as u32;
    let new_y = (config.y as f32 * height_ratio).round() as u32;
    let new_width = (config.width as f32 * width_ratio).round() as u32;
    let new_height = (config.height as f32 * height_ratio).round() as u32;

    (new_x, new_y, new_width, new_height)
}

#[tauri::command(async)]
pub async fn crop_image<R: Runtime>(app: AppHandle<R>, base64_image: String, region: CropRegion) -> Result<String, String> {
    let app_handle = app.clone();
    
    // Spawn a new thread for image cropping
    tokio::task::spawn_blocking(move || {
        let result = process_crop(&base64_image, region);
        
        match result {
            Ok(cropped_image) => {
                // Emit an event when cropping is complete
                if let Err(e) = app_handle.emit("crop-complete", &cropped_image) {
                    println!("Failed to emit crop complete event: {}", e);
                }
                Ok(cropped_image)
            }
            Err(e) => {
                // Emit an error event
                if let Err(emit_err) = app_handle.emit("crop-error", e.to_string()) {
                    println!("Failed to emit crop error event: {}", emit_err);
                }
                Err(e.to_string())
            }
        }
    }).await.unwrap_or_else(|e| Err(e.to_string()))
}

fn process_crop(base64_image: &str, region: CropRegion) -> Result<String, ImageError> {
    // Decode base64 image
    let image_data = BASE64.decode(base64_image).map_err(|e| {
        ImageError::IoError(std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    })?;

    // Load the image
    let mut img = image::load_from_memory(&image_data)?;
    
    // Get dimensions
    let (width, height) = img.dimensions();
    
    // Get crop configuration
    let config = get_crop_config(region);
    
    // Calculate proportional dimensions
    let (x, y, crop_width, crop_height) = calculate_proportional_dimensions(width, height, &config);
    
    // Crop the image
    let cropped = img.crop(x, y, crop_width, crop_height);
    
    // Save the cropped image as PNG for viewing
    // Create debug directory if it doesn't exist
    std::fs::create_dir_all("debug").map_err(|e| ImageError::IoError(e))?;
    let filename = format!("debug/{:?}.png", region);
    cropped.save(&filename)?;

    let mut buffer = Cursor::new(Vec::new());
    cropped.write_to(&mut buffer, ImageFormat::Png)?;
    let encoded = BASE64.encode(buffer.into_inner());
    
    Ok(encoded)
}