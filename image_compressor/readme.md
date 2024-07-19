# Image Compressor

This image compression compresses images in a specified folder using Rust and the `image` crate. The tool is designed to handle the processing in parallel, leveraging the `rayon` crate to improve performance by utilizing multi-core processors.

## Features

- **Parallel Processing**: Uses `rayon` for concurrent processing of multiple images.
- **Image Decoding and Encoding**: Supports various image formats for input and outputs them as compressed JPEG files.
- **Custom Compression Quality**: Allows specifying the JPEG quality for the output images.

## Requirements

- Rust Programming Language
- Cargo (Rust's package manager)

## Dependencies

To run this program, you need to include the following dependencies in your `Cargo.toml`:

```toml```
[dependencies]
image = "0.23"
rayon = "1.5"

Compress_image
 - This function takes three parameters: 
 - source (path to the image to be compressed), 
 - destination (path where the compressed image will be saved), 
 - and quality (compression quality as a value between 0 and 100).
Compress_folder 
 - Setting up parallel processing for the compression of multiple images.
Project_structure
├── dest_dir 
│   ├── picture1.jpg -> this will come after compress file from soruce_dir
│   ├── picture2.jpg -> this will come after compress file from soruce_dir
├── source_dir
│   ├── picture1.png
│   ├── picture2.gif
├── src
│   ├── main.rs
├── Cargo.lock
├── Cargo.toml

Function Parameters

source: The path to the directory containing the original images.
dest: The path to the directory where the compressed images will be stored.
quality: An integer value (0-100) specifying the compression quality for the JPEG output.

let source = PathBuf::from("source_dir");
let destination = PathBuf::from("dest_dir");
let quality = 80;

compress_folder(&source, &destination, quality);
This example compresses all images in source_dir and stores the compressed JPEG files in dest_dir with a quality setting of 80.

To build : cargo build --release

To run : cargo run -- source_dir/ dest_dir/

