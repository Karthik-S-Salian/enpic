// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod inference;
use base64::{engine::general_purpose, Engine as _};
use std::io::Cursor;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command(async)]
fn process_images(images: Vec<Vec<u8>>) -> Result<Vec<String>, String> {
    let mut results = Vec::new();

    for image_data in images {
        let img = image::load_from_memory(&image_data).map_err(|e| e.to_string())?;

        println!("{}", img.width());

        let upscaled = inference::upscale_image(img).map_err(|e| e.to_string())?;

        let mut cursor = Cursor::new(Vec::new());
        upscaled
            .write_to(&mut cursor, image::ImageFormat::Jpeg)
            .map_err(|e| e.to_string())?;

        let buf = cursor.into_inner();
        let encoded = general_purpose::STANDARD.encode(buf);
        results.push(format!("data:image/jpeg;base64,{}", encoded));
    }

    Ok(results)
}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, process_images])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
