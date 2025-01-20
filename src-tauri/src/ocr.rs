use std::error::Error;
use std::sync::Arc;

use ocrs::{ImageSource, OcrEngine, OcrEngineParams};
use rten::Model;
#[allow(unused)]
use rten_tensor::prelude::*;

use base64::{Engine, engine::general_purpose::STANDARD};
use image::{self, ImageBuffer, Rgb};
use tauri::{AppHandle, Manager, path::BaseDirectory, Runtime, Emitter};

#[tauri::command(async)]
pub async fn perform_ocr<R: Runtime>(app: AppHandle<R>, base64_image: String) -> Result<Vec<String>, String> {
    let app_handle = app.clone();
    
    // Spawn a new thread for OCR processing
    tokio::task::spawn_blocking(move || {
        let result = process_ocr(&app_handle, &base64_image);
        
        match result {
            Ok(texts) => {
                // Emit an event when OCR is complete
                if let Err(e) = app_handle.emit("ocr-complete", texts.clone()) {
                    println!("Failed to emit OCR complete event: {}", e);
                }
                Ok(texts)
            }
            Err(e) => {
                // Emit an error event
                if let Err(emit_err) = app_handle.emit("ocr-error", e.to_string()) {
                    println!("Failed to emit OCR error event: {}", emit_err);
                }
                Err(e.to_string())
            }
        }
    }).await.unwrap_or_else(|e| Err(e.to_string()))
}

fn process_ocr<R: Runtime>(app: &AppHandle<R>, base64_image: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let detection_model_path = app.path().resolve("resources/ai_models/text-detection.rten", BaseDirectory::Resource)?;
    let rec_model_path = app.path().resolve("resources/ai_models/text-recognition.rten", BaseDirectory::Resource)?;
    println!("Detection model path: {}", detection_model_path.display());
    println!("Recognition model path: {}", rec_model_path.display());

    // Emit progress event
    let _ = app.emit("ocr-progress", "Loading models...");
    
    let detection_model = Model::load_file(detection_model_path)?;
    let recognition_model = Model::load_file(rec_model_path)?;

    let engine = OcrEngine::new(OcrEngineParams {
        detection_model: Some(detection_model),
        recognition_model: Some(recognition_model),
        ..Default::default()
    })?;

    // Emit progress event
    let _ = app.emit("ocr-progress", "Processing image...");

    // Decode base64 image
    let image_data = STANDARD.decode(base64_image)?;
    let img = image::load_from_memory(&image_data)?.into_rgb8();

    let img_source = ImageSource::from_bytes(img.as_raw(), img.dimensions())?;
    let ocr_input = engine.prepare_input(img_source)?;

    // Emit progress event
    let _ = app.emit("ocr-progress", "Detecting text...");

    let word_rects = engine.detect_words(&ocr_input)?;
    let line_rects = engine.find_text_lines(&ocr_input, &word_rects);
    
    // Emit progress event
    let _ = app.emit("ocr-progress", "Recognizing text...");
    
    let line_texts = engine.recognize_text(&ocr_input, &line_rects)?;

    let mut results = Vec::new();
    for line in line_texts
        .iter()
        .flatten()
        .filter(|l| l.to_string().len() > 1)
    {
        println!("{}", line);
        results.push(line.to_string());
    }

    Ok(results)
}
