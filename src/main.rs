use opencv::{
    core::{Mat, CV_8UC1},
    imgcodecs,
    imgproc,
    prelude::*,
    Result,
};
use tesseract::Tesseract;

fn main() -> Result<()> {
    // Load your screenshot (adjust path as needed)
    let img = imgcodecs::imread("screenshot.png", imgcodecs::IMREAD_COLOR)?;

    // Convert to grayscale
    let mut gray = Mat::default();
    imgproc::cvt_color(&img, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

    // Apply thresholding (tweak values as needed)
    let mut binary = Mat::default();
    imgproc::threshold(&gray, &mut binary, 150.0, 255.0, imgproc::THRESH_BINARY_INV)?;

    // Optional: crop a region where the time is (adjust coordinates)
    let roi = Mat::roi(&binary, opencv::core::Rect::new(300, 100, 200, 50))?;

    // Save temp image to pass to Tesseract
    imgcodecs::imwrite("roi.png", &roi, &opencv::types::VectorOfi32::new())?;

    // Run Tesseract on cropped image
    let text = Tesseract::new(None, "eng")
        .and_then(|mut tess| {
            tess.set_image("roi.png")?;
            tess.set_page_seg_mode(tesseract::PageSegMode::SingleLine);
            tess.get_text()
        })
        .expect("Failed to run Tesseract");

    println!("OCR output: {}", text);

    Ok(())
}