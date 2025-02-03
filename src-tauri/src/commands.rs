use crate::{crop, ocr};
use tauri::Manager;

#[tauri::command]
pub async fn submit_screenshot(app_handle: tauri::AppHandle, screenshot_id: i32) -> Result<(), String> {
    use crate::models::screenshots::dsl::{screenshots, id, image, recognized};
    use crate::models::screenshots::columns::ocr as ocr_column;
    use diesel::prelude::*;

    // Get database connection from app state
    let app_state = app_handle.state::<crate::AppState>();
    let db = app_state.inner().db.as_ref().ok_or("Database not initialized")?;
    
    // Get screenshot data first, then drop the connection
    let screenshot_data: String = {
        let mut conn = db.lock().map_err(|_| "Failed to lock database connection")?;
        screenshots
            .filter(id.eq(screenshot_id))
            .select(image)
            .first(&mut *conn)
            .map_err(|e| format!("Failed to get screenshot: {}", e))?
    };

    println!("Processing screenshot ID: {}", screenshot_id);

    // Define regions to process
    let regions = vec![
        (crop::CropRegion::SummaryFirst, "First Summary"),
        (crop::CropRegion::SummarySecond, "Second Summary"),
        (crop::CropRegion::SummaryThird, "Third Summary"),
        (crop::CropRegion::SummaryFourth, "Fourth Summary"),
        (crop::CropRegion::SummaryUsername, "Username"),
    ];

    // Process each region
    for (region, region_name) in regions {
        // Crop the region
        let cropped_image = crop::crop_image(app_handle.clone(), screenshot_data.clone(), region)
            .await
            .map_err(|e| format!("Failed to crop {}: {}", region_name, e))?;

        // Perform OCR on the cropped region
        let ocr_results = ocr::perform_ocr(app_handle.clone(), cropped_image)
            .await
            .map_err(|e| format!("Failed OCR for {}: {}", region_name, e))?;

        // Print OCR results
        println!("OCR Results for {}: ", region_name);
        for line in ocr_results {
            println!("  {}", line);
        }
    }

    // Update the screenshot record
    {
        let mut conn = db.lock().map_err(|_| "Failed to lock database connection")?;
        diesel::update(screenshots.filter(id.eq(screenshot_id)))
            .set((
                recognized.eq(true),
                ocr_column.eq(true),
            ))
            .execute(&mut *conn)
            .map_err(|e| format!("Failed to update screenshot status: {}", e))?;
    }

    Ok(())
}