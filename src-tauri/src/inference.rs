use image::{DynamicImage, RgbImage};
use ndarray::Array4;
use ort::execution_providers::{
    CPUExecutionProvider, CUDAExecutionProvider, OpenVINOExecutionProvider,
    TensorRTExecutionProvider,
};
use ort::session::{builder::GraphOptimizationLevel, Session};
use std::sync::OnceLock;

static UPSCALER_SESSION: OnceLock<Session> = OnceLock::new();

pub fn upscale_image(image: DynamicImage) -> Result<RgbImage, Box<dyn std::error::Error>> {
    let session = UPSCALER_SESSION.get_or_init(|| {
        Session::builder()
            .unwrap()
            .with_execution_providers([
                // Try GPU providers first
                TensorRTExecutionProvider::default().build(),
                CUDAExecutionProvider::default().build().error_on_failure(),
                // // Then CPU-accelerated EPs
                // OpenVINOExecutionProvider::default().build(),
                // // Finally, default CPU EP
                // CPUExecutionProvider::default().build(),
            ])
            .unwrap()
            .with_optimization_level(GraphOptimizationLevel::Level3)
            .unwrap()
            .with_intra_threads(16)
            .unwrap()
            .commit_from_file(
                "/home/karthikssalian/work/enpic/models/2x.StarSample.V1.0.-.FP32.OPSET17.onnx",
            )
            .expect("could not initialize model")
    });

    println!("here");

    // Preprocess
    let rgb = image.to_rgb8();
    let (w, h) = rgb.dimensions();
    let mut input = Array4::<f32>::zeros((1, 3, h as usize, w as usize));

    for (x, y, pixel) in rgb.enumerate_pixels() {
        let [r, g, b] = pixel.0;
        input[[0, 0, y as usize, x as usize]] = r as f32 / 255.0;
        input[[0, 1, y as usize, x as usize]] = g as f32 / 255.0;
        input[[0, 2, y as usize, x as usize]] = b as f32 / 255.0;
    }

    // Run inference
    let outputs = session.run(ort::inputs![input]?)?;

    let inter = outputs.values().next().ok_or("Model returned no output")?;
    let output_tensor = inter.try_extract_tensor::<f32>()?;
    let out = output_tensor.view();

    // Convert back to RgbImage
    let (_, _, out_h, out_w) = (
        out.shape()[0],
        out.shape()[1],
        out.shape()[2],
        out.shape()[3],
    );
    let mut result = RgbImage::new(out_w as u32, out_h as u32);

    for y in 0..out_h {
        for x in 0..out_w {
            let r = (out[[0, 0, y, x]].clamp(0.0, 1.0) * 255.0) as u8;
            let g = (out[[0, 1, y, x]].clamp(0.0, 1.0) * 255.0) as u8;
            let b = (out[[0, 2, y, x]].clamp(0.0, 1.0) * 255.0) as u8;
            result.put_pixel(x as u32, y as u32, image::Rgb([r, g, b]));
        }
    }

    Ok(result)
}
