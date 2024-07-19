# Image filtering (blurring and sharpening)

This project processes images using Rust and the 'image' crate, including functionality for blurring and sharpening images. The project leverages the 'rayon' crate to handle the processing in parallel, improving performance by utilizing multi-core processors.

## Features

- **Parallel Processing: Uses 'rayon' for concurrent processing of image chunks.
- **Image Decoding and Encoding: Supports various image formats for input and outputs them as processed image files.
- **Blurring Function: Applies Gaussian blur to images.
- **Sharpening Function: Enhances image edges using a sharpening filter.

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

## compress_image
 - This function takes three parameters: 
 - source (path to the image to be compressed), 
 - destination (path where the compressed image will be saved), 
 - and quality (compression quality as a value between 0 and 100).

## compress_folder 
 - Setting up parallel processing for the compression of multiple images.

## Projectstructre
# ├── dest_dir 
# │   ├── picture1.jpg -> this will come after compress file from soruce_dir
# │   ├── picture2.jpg -> this will come after compress file from soruce_dir
# ├── source_dir
# │   ├── picture1.png
# │   ├── picture2.gif
# ├── src
# │   ├── main.rs
# ├── Cargo.lock
# ├── Cargo.toml

# Function Parameters

source: The path to the directory containing the original images.
dest: The path to the directory where the compressed images will be stored.
quality: An integer value (0-100) specifying the compression quality for the JPEG output.

let source = PathBuf::from("source_dir");
let destination = PathBuf::from("dest_dir");
let quality = 80;

compress_folder(&source, &destination, quality);
This example compresses all images in source_dir and stores the compressed JPEG files in dest_dir with a quality setting of 80.

## To build : cargo build --release

## To run : cargo run -- source_dir/ dest_dir/
