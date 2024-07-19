# Image filtering (blurring and sharpening)

This project processes images using Rust and the 'image' crate, including functionality for blurring and sharpening images. The project leverages the 'rayon' crate to handle the processing in parallel, improving performance by utilizing multi-core processors.

## Features

- **Parallel Processing**: Uses 'rayon' for concurrent processing of image chunks.
- **Image Decoding and Encoding**: Supports various image formats for input and outputs them as processed image files.
- **Blurring Function**: Applies Gaussian blur to images.
- **Sharpening Function**: Enhances image edges using a sharpening filter.

## Requirements

- Rust Programming Language
- Cargo (Rust's package manager)

## Dependencies

To run this program, you need to include the following dependencies in your `Cargo.toml`:

```toml
[dependencies]
image = "0.25.1"
imageproc = "0.25.0"
rayon = "1.5"
```
## Functions for blurring
 - The `image_to_chunks` function divides the input image into smaller chunks. For each loop, a chunk from the original image with the specified width and height is created. Finally, each chunk is added to a vector.
 - The `gaussian_blur_chunk` function is where Gaussian blur is applied to each chunk.
 - The `par_blurring` function calls `gaussian_blur_chunk` function in parallel, applying Gaussian blur to each chunk. The parallel technique used is Data Parallelism.
 - The `normal_blurring` function is exactly like `par_blurring` without the parallel aspect.
 - The `chunks_to_image` function takes the blurred chunks, then reconstructs them into a blurred image using the width and height from the original image.

## Functions for sharpening
 - The `sharpen` function performs image sharpening but in a non-parallel manner. There is a loop that iterates over each pixel, applying the sharpening filter by summing the weighted values of the neighboring pixels.
 - The `par_sharpen` function performs image sharpening but in a parallel manner. The parallel technique used is Data Parallelism. The function begins by converting the height range into the parallel iterator. Then a nested map call iterates over each pixel in parallel, applying the sharpening filter.

## Project structre
# ├── src
# │   ├── image_blurring.rs
# │   ├── image_sharpening.rs.rs
# │   ├── lib.rs
# │   ├── main.rs
# ├── Cargo.lock
# ├── Cargo.toml

## How to run the functions

It is very important that you are in the image_filtering directory to run the function. You can check the `main.rs` file to change the image that you want to blur or sharpen, you are also able to change the strength of the blur as well. It is recommended to use a lower blur strenth like 3.0 or 4.0 for images with less pixels. However, for a 4k image, it is recommended to use a blur strength of 8.0.

## To build : cargo build

## To run : cargo run
