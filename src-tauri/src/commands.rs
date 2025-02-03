use crate::{crop, ocr};
use tauri::Manager;

#[tauri::command]
pub async fn submit_screenshot(app_handle: tauri::AppHandle, screenshot_id: i32) -> Result<(), String> {
    use crate::models::screenshots::dsl::{screenshots, id, image, recognized, summary_first, summary_second, summary_third, summary_fourth, summary_username};
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

    // Process each region and collect results
    let mut first_summary = String::new();
    let mut second_summary = String::new();
    let mut third_summary = String::new();
    let mut fourth_summary = String::new();
    let mut username = String::new();

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

        // Store results in the appropriate variable
        let result_text = ocr_results.join(" ");
        match region {
            crop::CropRegion::SummaryFirst => first_summary = result_text.clone(),
            crop::CropRegion::SummarySecond => second_summary = result_text.clone(),
            crop::CropRegion::SummaryThird => third_summary = result_text.clone(),
            crop::CropRegion::SummaryFourth => fourth_summary = result_text.clone(),
            crop::CropRegion::SummaryUsername => username = result_text.clone(),
            _ => (), // Ignore any other regions
        }

        // Print OCR results
        println!("OCR Results for {}: {}", region_name, result_text);
    }

    // Update the screenshot record with all OCR results
    {
        let mut conn = db.lock().map_err(|_| "Failed to lock database connection")?;
        diesel::update(screenshots.filter(id.eq(screenshot_id)))
            .set((
                recognized.eq(true),
                ocr_column.eq(true),
                summary_first.eq(first_summary),
                summary_second.eq(second_summary),
                summary_third.eq(third_summary),
                summary_fourth.eq(fourth_summary),
                summary_username.eq(username),
            ))
            .execute(&mut *conn)
            .map_err(|e| format!("Failed to update screenshot status: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
pub async fn reload_shortcut(app_handle: tauri::AppHandle) -> Result<(), String> {
    use tauri_plugin_global_shortcut::GlobalShortcutExt;

    // Unregister all existing shortcuts
    app_handle.global_shortcut().unregister_all()
        .map_err(|e| format!("Failed to unregister shortcuts: {}", e))?;

    // Re-register shortcuts using the existing function
    crate::shortcuts::register_shortcuts(&app_handle)
        .map_err(|e| format!("Failed to register shortcuts: {}", e))?;

    Ok(())
}