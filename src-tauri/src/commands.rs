#[tauri::command]
pub fn submit_screenshot(screenshot_id: i32) {
    println!("I was invoked from JavaScript! {}", screenshot_id);
}