// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn process_images(images: Vec<Vec<u8>>) -> Result<Vec<Vec<u8>>, String> {
    let mut results = Vec::new();

    for image_data in images {
        let img = image::load_from_memory(&image_data).map_err(|e| e.to_string())?;

        println!("{}",img.width());

        // // Run ONNX inference using ort crate here
        // let upscaled = upscale_image(img)
        //     .map_err(|e| e.to_string())?;

        // // Convert result to Vec<u8> (e.g., PNG)
        // let mut buf = Vec::new();
        // upscaled.write_to(&mut buf, image::ImageOutputFormat::Png)
        //     .map_err(|e| e.to_string())?;

        // results.push(buf);
    }

    Ok(results)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet,process_images])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
