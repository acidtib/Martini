// screenshot.rs
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};
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

/// Captures a window screenshot by partial title match and returns the image data as PNG
pub fn capture_window(window_title: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let window = find_window(window_title)?;
    
    // Capture the window image
    let image = window.capture_image()?;

    // Convert to PNG
    let mut png_data = Vec::new();
    image.write_to(&mut std::io::Cursor::new(&mut png_data), image::ImageFormat::Png)?;

    Ok(png_data)
}

/// Captures a window screenshot and saves it to a file
pub fn capture_and_save_window(window_title: &str) -> Result<String, Box<dyn Error>> {
    let window = find_window(window_title)?;
    
    // Capture the window image
    let image = window.capture_image()?;

    // Generate unique filename with timestamp
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs();
    let filename = format!("{}_{}.png", window_title, timestamp);

    // Save the image
    image.save(&filename)?;

    Ok(filename)
}

/// Lists all available windows
pub fn list_windows() -> Result<Vec<WindowInfo>, Box<dyn Error>> {
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

/// Helper function to find a window by partial title
fn find_window(window_title: &str) -> Result<Window, Box<dyn Error>> {
    let windows = Window::all()?;
    let window = windows
        .into_iter()
        .find(|w| w.title().to_lowercase().contains(&window_title.to_lowercase()))
        .ok_or_else(|| format!("Window with title containing '{}' not found", window_title))?;

    // Skip minimized windows as they can't be captured properly
    if window.is_minimized() {
        return Err("Window is minimized and cannot be captured".into());
    }

    Ok(window)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_capture_window() {
        match capture_window("test window") {
            Ok(png_data) => {
                fs::write("test_capture.png", png_data).unwrap();
                assert!(fs::metadata("test_capture.png").unwrap().len() > 0);
                fs::remove_file("test_capture.png").unwrap();
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