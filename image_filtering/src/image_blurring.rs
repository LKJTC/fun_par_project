use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, Rgba};
use imageproc::filter::gaussian_blur_f32;
use rayon::prelude::*;
use std::time::Instant;

/*
* I have completed the blurring portion of image processing.
* Me and teammate have completed 50% of our project currently.
* There are some issues that needs to be fixed. The final blurred image has lines, due to the division of chunks.
*/
#[allow(dead_code)]
pub fn get_image(path: &str) -> DynamicImage {
    ImageReader::open(path)
        .expect("Failed to open image")
        .with_guessed_format()
        .expect("Failed to guess image format")
        .decode()
        .expect("Failed to decode image")
}

/*
* Divides the input image into smaller chunks of the specified input height.
* For each loop, a chunk from the original image with the specified width and height
* is created.
* Finally, each chunk is added to a vector called chunks and is returned.
*/
#[allow(dead_code)]
pub fn image_to_chunks(
    image: &DynamicImage,
    chunk_height: u32,
) -> Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let (width, height) = image.dimensions();
    let mut chunks = Vec::new();
    for y in (0..height).step_by(chunk_height as usize) {
        let sub_img: ImageBuffer<Rgba<u8>, Vec<u8>> = image
            .view(0, y, width, chunk_height.min(height - y))
            .to_image();
        chunks.push(sub_img);
    }
    chunks
}

/*
* This function takes the chunks vector from the previous function as input, and
* returns a vector of blurred chunks.
*/
#[allow(dead_code)]
pub fn gaussian_blur_chunk(
    chunk: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    sigma: f32,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    gaussian_blur_f32(chunk, sigma)
}

// Parallel blurring version, the parallel technique used is Data Parallelism.
#[allow(dead_code)]
pub fn par_blurring(
    chunks: Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>,
    sigma: f32,
) -> Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    chunks
        .into_par_iter()
        .map(|chunk| gaussian_blur_chunk(&chunk, sigma))
        .collect()
}

// Non-parallel blurring version
#[allow(dead_code)]
pub fn normal_blurring(
    chunks: Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>,
    sigma: f32,
) -> Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    chunks
        .into_iter()
        .map(|chunk| gaussian_blur_chunk(&chunk, sigma))
        .collect()
}

/*
* This function takes a reference to the blurred chunks vector, with
* reference to the height and width from the original image, and returns a single
* ImageBuffer.
* It loops over the blurred chunks and copies each chunk into the appropriate position
* in the new ImageBuffer.
* After this process is over, the reconstructed iamge is returned.
*/
#[allow(dead_code)]
pub fn chunks_to_image(
    chunks: &[ImageBuffer<Rgba<u8>, Vec<u8>>],
    width: u32,
    height: u32,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut image = ImageBuffer::new(width, height);
    let mut y_offset = 0;
    for chunk in chunks {
        image
            .copy_from(chunk, 0, y_offset)
            .expect("Failed to copy chunk");
        y_offset += chunk.height();
    }
    image
}

#[allow(dead_code)]
pub fn save_image(image: &DynamicImage, path: &str) {
    image.save(path).expect("save_image");
}
