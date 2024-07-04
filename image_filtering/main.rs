use image::{open, DynamicImage, GenericImageView};
use imageproc::filter::gaussian_blur_f32;
use rayon::prelude::*;

fn get_image(path: &str) -> DynamicImage {
    open(path).expect("get_image")
}

fn image_to_chunks(image: &DynamicImage, chunk_height: u32) -> Vec<DynamicImage> {
    let (width, height) = image.dimensions();
    let mut chunks = Vec::new();

    for y in (0..height).step_by(chunk_height as usize) {
        let sub_img = image.view(0, y, width, chunk_height.min(height - y)).to_image();
        chunks.push(sub_img);
    }
    chunks
}

fn gaussian_blur(chunks: Vec<DynamicImage>, blur_radius: f32) -> Vec<DynamicImage> {
    let blur_radius = blur_radius.max(1.0);
    chunks
        .into_par_iter()
        .map(|chunk| gaussian_blur_f32(&chunk, blur_radius))
        .collect()
}

fn chunks_to_image(chunks: Vec<DynamicImage>, width: u32, height: u32) -> DynamicImage {
    let mut image = DynamicImage::new_luma8(width, height);
    for (y, chunk) in chunks.into_iter().enumerate() {
        image.copy_from(&chunk, 0, (y as u32) * (height / chunks.len() as u32)).unwrap();
    }
    image
}

fn save_image(image: &DynamicImage, path: &str) {
    image.save(path).expect("save_image");
}

fn main() {
    let image = get_image("burger.png");
    let (width, height) = image.dimensions();
    let chunk_height = height/4;
    let chunks = image_to_chunks(&image, chunk_height);
    let blurred_chunks = gaussian_blur(chunks, 10.0);
    let blurred_image = chunks_to_image(blurred_chunks, width, height);
    save_image(&blurred_image, "blurred_image.png");
}