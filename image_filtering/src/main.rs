use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba, GenericImage};
use imageproc::filter::gaussian_blur_f32;
use rayon::prelude::*;
use image::io::Reader as ImageReader;

/*
* I have completed the blurring portion of image processing.
* Me and teammate have completed 50% of our project currently.
* There are some issues that needs to be fixed. The final blurred image has lines, due to the division of chunks.
*/

fn get_image(path: &str) -> DynamicImage {
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
fn image_to_chunks(image: &DynamicImage, chunk_height: u32) -> Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let (width, height) = image.dimensions();
    let mut chunks = Vec::new();
    for y in (0..height).step_by(chunk_height as usize) {
        let sub_img: ImageBuffer<Rgba<u8>, Vec<u8>> = image.view(0, y, width, chunk_height.min(height - y)).to_image();
        chunks.push(sub_img);
    }
    chunks
}

/*
* This function takes the chunks vector from the previous function as input, and
* returns a vector of blurred chunks.
* It uses parallel iteration to apply Gaussian Blur to each chunk using the blur radius.
*/
fn gaussian_blur(chunks: Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>, blur_radius: f32) -> Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let blur_radius = blur_radius.max(1.0);
    chunks
        .into_par_iter()
        .map(|chunk| gaussian_blur_f32(&chunk, blur_radius))
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
fn chunks_to_image(chunks: &[ImageBuffer<Rgba<u8>, Vec<u8>>], width: u32, height: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut image = ImageBuffer::new(width, height);
    let mut y_offset = 0;
    for chunk in chunks {
        image.copy_from(chunk, 0, y_offset).expect("Failed to copy chunk");
        y_offset += chunk.height();
    }
    image
}

fn save_image(image: &DynamicImage, path: &str) {
    image.save(path).expect("save_image");
}

fn main() {
    let image = get_image("burger.png");
    let (width, height) = image.dimensions();
    let chunk_height = (height+3) / 4;
    let chunks = image_to_chunks(&image, chunk_height);
    let blurred_chunks = gaussian_blur(chunks, 10.0);
    let blurred_image = chunks_to_image(&blurred_chunks, width, height);
    save_image(&DynamicImage::ImageRgba8(blurred_image), "blurred_image.png");
}