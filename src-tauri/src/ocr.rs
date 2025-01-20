use std::error::Error;

use ocrs::{ImageSource, OcrEngine, OcrEngineParams};
use rten::Model;
#[allow(unused)]
use rten_tensor::prelude::*;

use base64::{Engine, engine::general_purpose::STANDARD};
use image::{self, ImageBuffer, Rgb};
use tauri::{AppHandle, Manager, path::BaseDirectory};

pub fn perform_ocr(app: &AppHandle, base64_image: &str) -> Result<(), Box<dyn Error>> {
    let detection_model_path = app.path().resolve("resources/ai_models/text-detection.rten", BaseDirectory::Resource)?;
    let rec_model_path = app.path().resolve("resources/ai_models/text-recognition.rten", BaseDirectory::Resource)?;
    println!("Detection model path: {}", detection_model_path.display());
    println!("Recognition model path: {}", rec_model_path.display());

    let detection_model = Model::load_file(detection_model_path)?;
    let recognition_model = Model::load_file(rec_model_path)?;

    let engine = OcrEngine::new(OcrEngineParams {
        detection_model: Some(detection_model),
        recognition_model: Some(recognition_model),
        ..Default::default()
    })?;

    // Decode base64 image
    let image_data = STANDARD.decode(base64_image)?;
    let img = image::load_from_memory(&image_data)?.into_rgb8();

    let img_source = ImageSource::from_bytes(img.as_raw(), img.dimensions())?;
    let ocr_input = engine.prepare_input(img_source)?;

    let word_rects = engine.detect_words(&ocr_input)?;
    let line_rects = engine.find_text_lines(&ocr_input, &word_rects);
    let line_texts = engine.recognize_text(&ocr_input, &line_rects)?;

    for line in line_texts
        .iter()
        .flatten()
        .filter(|l| l.to_string().len() > 1)
    {
        println!("{}", line);
    }

    Ok(())
}
