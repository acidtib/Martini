// screenshot.rs
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use image::ImageError;
use xcap::Window;

/// Structure to hold window information
#[derive(Debug)]
pub struct WindowInfo {
    pub title: String,
    pub app_name: String,
    pub dimensions: (u32, u32),
    pub position: (i32, i32),
    pub is_minimized: bool,
    pub is_maximized: bool,
}

/// Captures a window screenshot by partial title match and returns the image data as JPEG
pub fn capture_window(window_titles: &[&str]) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
    let window = find_window(window_titles)?;

    // Capture the window image
    let image = window.capture_image()?;

    // Convert to JPEG with quality settings
    let mut jpeg_data = Vec::new();
    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut jpeg_data, 95);
    encoder.encode_image(&image)?;

    // Save the screenshot as JPEG
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    std::fs::create_dir_all("../debug_images").map_err(|e| ImageError::IoError(e))?;
    let filename = format!("../debug_images/screenshot_{}.jpg", timestamp);
    std::fs::write(&filename, &jpeg_data)?;

    Ok(jpeg_data)
}

/// Captures a window screenshot and saves it to a file
pub fn capture_and_save_window(window_titles: &[&str]) -> Result<String, Box<dyn Error + Send + Sync>> {
    let jpeg_data = capture_window(window_titles)?;
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let filename = format!("screenshot_{}.jpg", timestamp);

    // Save the image
    std::fs::write(&filename, jpeg_data)?;

    Ok(filename)
}

/// Lists all available windows
pub fn list_windows() -> Result<Vec<WindowInfo>, Box<dyn Error + Send + Sync>> {
    let windows = Window::all()?;
    Ok(windows
        .into_iter()
        .map(|w| WindowInfo {
            title: w.title().to_string(),
            app_name: w.app_name().to_string(),
            dimensions: (w.width(), w.height()),
            position: (w.x(), w.y()),
            is_minimized: w.is_minimized(),
            is_maximized: w.is_maximized(),
        })
        .collect())
}

/// Helper function to find a window by trying multiple partial title matches
fn find_window(window_titles: &[&str]) -> Result<Window, Box<dyn Error + Send + Sync>> {
    let windows = Window::all()?;

    for window in windows {
        let title = window.title().to_string().to_lowercase();
        
        // Try to match any of the provided titles
        if window_titles.iter().any(|&t| title.contains(&t.to_lowercase())) {
            return Ok(window);
        }
    }

    Err("No matching window found".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_capture_window() {
        match capture_window(&["test window"]) {
            Ok(jpeg_data) => {
                fs::write("test_capture.jpg", jpeg_data).unwrap();
                assert!(fs::metadata("test_capture.jpg").unwrap().len() > 0);
                fs::remove_file("test_capture.jpg").unwrap();
            }
            Err(e) => println!("Test skipped: {}", e),
        }
    }

    #[test]
    fn test_list_windows() {
        let windows = list_windows().unwrap();
        assert!(!windows.is_empty(), "Should find at least one window");
    }
}
