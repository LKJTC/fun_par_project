use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use rayon::prelude::*;
use std::path::Path;
use std::time::Instant;

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
This function performs image sharpening but in a non-parallel manner.
Basically, the loops iterate over each pixel, applying the sharpening filter
by summing the weighted values of the neighboring pixels.
Finally, the calculated pixel values are written to the output_img buffer.
*/
#[allow(dead_code)]
pub fn sharpen(img: &DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let mut output_img = ImageBuffer::new(width, height);

    let offsets = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (0, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let mut r = 0i32;
            let mut g = 0i32;
            let mut b = 0i32;

            for &(dx, dy) in &offsets {
                let pixel = img.get_pixel((x as i32 + dx) as u32, (y as i32 + dy) as u32);
                let weight = if dx == 0 && dy == 0 { 9 } else { -1 };
                r += pixel[0] as i32 * weight;
                g += pixel[1] as i32 * weight;
                b += pixel[2] as i32 * weight;
            }

            let pixel = Rgba([
                r.clamp(0, 255) as u8,
                g.clamp(0, 255) as u8,
                b.clamp(0, 255) as u8,
                img.get_pixel(x, y)[3],
            ]);

            output_img.put_pixel(x, y, pixel);
        }
    }

    output_img
}

/*
This function performs image sharpening, but in parallel manner using rayon.
Basically, the function begins by converting the height range into the parallel iterator.
Then a nested map call iterates over each pixel in parallel, applying the sharpening filter.
Finally, in a loop the calculated pixel values are written to the output_img_buffer.
*/
#[allow(dead_code)]
pub fn par_sharpen(img: &DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();

    let offsets = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (0, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let output_img: Vec<Vec<Rgba<u8>>> = (0..height)
        .into_par_iter()
        .map(|y| {
            (0..width)
                .map(|x| {
                    if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                        return img.get_pixel(x, y);
                    }

                    let mut r = 0i32;
                    let mut g = 0i32;
                    let mut b = 0i32;

                    for &(dx, dy) in &offsets {
                        let pixel = img.get_pixel((x as i32 + dx) as u32, (y as i32 + dy) as u32);
                        let weight = if dx == 0 && dy == 0 { 9 } else { -1 };
                        r += pixel[0] as i32 * weight;
                        g += pixel[1] as i32 * weight;
                        b += pixel[2] as i32 * weight;
                    }

                    Rgba([
                        r.clamp(0, 255) as u8,
                        g.clamp(0, 255) as u8,
                        b.clamp(0, 255) as u8,
                        img.get_pixel(x, y)[3],
                    ])
                })
                .collect()
        })
        .collect();

    let mut output_img_buffer = ImageBuffer::new(width, height);
    for (y, row) in output_img.iter().enumerate() {
        for (x, &pixel) in row.iter().enumerate() {
            output_img_buffer.put_pixel(x as u32, y as u32, pixel);
        }
    }

    output_img_buffer
}

#[allow(dead_code)]
pub fn save_image(image: &DynamicImage, path: &str) {
    image.save(path).expect("Failed to save image");
}
