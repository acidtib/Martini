use image::{GenericImageView, ImageBuffer, ImageError, Rgb};
use image::ImageFormat;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use std::io::Cursor;
use std::fs;
use std::path::Path;
use chrono;

#[derive(Debug, Clone, Copy)]
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

pub fn crop_image(base64_image: &str, region: CropRegion) -> Result<String, ImageError> {
    // Decode base64 to bytes
    let image_bytes = BASE64.decode(base64_image).map_err(|_| {
        ImageError::Parameter(image::error::ParameterError::from_kind(
            image::error::ParameterErrorKind::Generic("Invalid base64 input".to_string())
        ))
    })?;

    // Load the image from bytes
    let img = image::load_from_memory(&image_bytes).map_err(|e| {
        println!("Failed to load image from base64: {}", e);
        e
    })?;

    let (orig_width, orig_height) = img.dimensions();
    let config = get_crop_config(region);

    let (x, y, crop_width, crop_height) =
        calculate_proportional_dimensions(orig_width, orig_height, &config);

    if x + crop_width > orig_width || y + crop_height > orig_height {
        return Err(ImageError::Parameter(
            image::error::ParameterError::from_kind(
                image::error::ParameterErrorKind::DimensionMismatch,
            ),
        ));
    }

    let mut cropped: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(crop_width, crop_height);

    for (dx, dy, pixel) in cropped.enumerate_pixels_mut() {
        let source_pixel = img.get_pixel(x + dx, y + dy);
        *pixel = Rgb([source_pixel[0], source_pixel[1], source_pixel[2]]);
    }

    // Save cropped image for debugging
    let debug_dir = Path::new("debug");
    if !debug_dir.exists() {
        fs::create_dir(debug_dir).map_err(|e| {
            println!("Failed to create debug directory: {}", e);
            ImageError::Parameter(image::error::ParameterError::from_kind(
                image::error::ParameterErrorKind::Generic("Failed to create debug directory".to_string())
            ))
        })?;
    }
    
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let debug_file = debug_dir.join(format!("crop_debug_{}.png", timestamp));
    cropped.save(&debug_file).map_err(|e| {
        println!("Failed to save debug image: {}", e);
        ImageError::Parameter(image::error::ParameterError::from_kind(
            image::error::ParameterErrorKind::Generic("Failed to save debug image".to_string())
        ))
    })?;
    
    println!("Saved debug image to: {:?}", debug_file);

    // Create a buffer to store the encoded image
    let mut buffer = Cursor::new(Vec::new());
    cropped.write_to(&mut buffer, ImageFormat::Png).map_err(|e| {
        println!("Failed to encode cropped image: {}", e);
        ImageError::Parameter(image::error::ParameterError::from_kind(
            image::error::ParameterErrorKind::Generic("Failed to encode image".to_string())
        ))
    })?;

    // Convert to base64
    let base64_output = BASE64.encode(buffer.into_inner());

    println!("Cropping region: {:?}", region);
    println!("Original dimensions: {}x{}", orig_width, orig_height);
    println!(
        "Cropped area: x={}, y={}, width={}, height={}",
        x, y, crop_width, crop_height
    );

    Ok(base64_output)
}