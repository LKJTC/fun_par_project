mod image_blurring;
mod image_sharpening;

use image::{DynamicImage, GenericImageView};
use image_blurring::{
    chunks_to_image, get_image as get_blur_image, image_to_chunks, normal_blurring, par_blurring,
    save_image as save_blur_image,
};
use image_sharpening::{
    get_image as get_sharpen_image, par_sharpen, save_image as save_sharpen_image, sharpen,
};

fn main() {
    // Blurring
    let blur_image = get_blur_image("4k_image.jpg");
    let (width, height) = blur_image.dimensions();
    let chunk_height = (height + 3) / 4;
    let chunks = image_to_chunks(&blur_image, chunk_height);

    // This is for 4k_image.jpg
    let start_par = std::time::Instant::now();
    let par_blurred_chunks = par_blurring(chunks, 8.0);
    let par_blurred_image = chunks_to_image(&par_blurred_chunks, width, height);
    let duration_par = start_par.elapsed();
    println!("Parallel blurring time taken: {:?}", duration_par);

    // This is for gojo.png
    // let start_par = std::time::Instant::now();
    // let par_blurred_chunks = par_blurring(chunks, 4.0);
    // let par_blurred_image = chunks_to_image(&par_blurred_chunks, width, height);
    // let duration_par = start_par.elapsed();
    // println!("Parallel blurring time taken: {:?}", duration_par);

    // Uncomment to run the non-parallel blurring
    // let start_normal = std::time::Instant::now();
    // let normal_blurred_chunks = normal_blurring(chunks, 8.0);
    // let normal_blurred_image = chunks_to_image(&normal_blurred_chunks, width, height);
    // let duration_normal = start_normal.elapsed();
    // println!("Normal blurring time taken: {:?}", duration_normal);

    save_blur_image(
        &image::DynamicImage::ImageRgba8(par_blurred_image),
        "blurred_image.png",
    );
    // save_blur_image(&image::DynamicImage::ImageRgba8(normal_blurred_image), "blurred_image.png");

    // Sharpening
    let sharpen_image = get_sharpen_image("landscape.jpg");

    let start_par = std::time::Instant::now();
    let par_sharpened_image = par_sharpen(&sharpen_image);
    let duration_par = start_par.elapsed();
    println!("Parallel sharpening time taken: {:?}", duration_par);

    // Uncomment to run the non-parallel sharpening
    // let start_normal = std::time::Instant::now();
    // let normal_sharpened_image = sharpen(&sharpen_image);
    // let duration_normal = start_normal.elapsed();
    // println!("Normal sharpening time taken: {:?}", duration_normal);

    let rgb_sharpened_image = image::DynamicImage::ImageRgba8(par_sharpened_image).to_rgb8();
    save_sharpen_image(
        &image::DynamicImage::ImageRgb8(rgb_sharpened_image),
        "sharpened_image.png",
    );
    // Uncomment this as well
    // let rgb_sharpened_image = image::DynamicImage::ImageRgba8(normal_sharpened_image).to_rgb8();
    // save_sharpen_image(&image::DynamicImage::ImageRgb8(rgb_sharpened_image), "sharpened_image.png");
}
