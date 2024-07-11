use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use image::io::Reader as ImageReader;
use image::codecs::jpeg::JpegEncoder;


// This function takes three parameters: 
// source (path to the image to be compressed), 
// destination (path where the compressed image will be saved), 
// and quality (compression quality as a value between 0 and 100).


fn compress_image(source: &Path, destination: &Path, quality: u8) {
    //Opens the image file from the given path.
    let img = ImageReader::open(source)
        .expect("Failed to open image")
        .decode() //Using .decode() to decodes the image file into a usable image format.
        .expect("Failed to decode image");
    // Create the destination file
    let mut output_file = File::create(destination)
        .expect("Failed to create output file");
    // Create a JPEG encoder with the specified quality
    let mut encoder = JpegEncoder::new_with_quality(&mut output_file, quality);
    // Encode the image as JPEG and save it
    encoder.encode_image(&img)
        .expect("Failed to encode and save image");
}

// setting up parallel processing for the compression of multiple images.

fn compress_folder(source: &Path, dest: &Path, quality: u8) {
    fs::create_dir_all(dest).expect("Failed to create destination folder");
    // Collect all file paths from the source directory
    let entries: Vec<_> = fs::read_dir(source)
        .expect("Failed to read source directory")
        .map(|entry| entry.expect("Failed to read directory entry").path())
        .collect();

    // Process each file in parallel
    entries.par_iter().for_each(|path| {
        if path.is_file() {
            let mut dest_path = PathBuf::from(dest);
            if let Some(filename) = path.file_name() {
                // Convert images from various formats (e.g., PNG, BMP, GIF)
                // Set new file name with .jpg extension
                dest_path.push(filename);
                dest_path.set_extension("jpg"); 
                compress_image(path, &dest_path, quality);
            }
        }
    });
    println!("Folder compression successful");
}

// # project_folder
// # ├── dest_dir 
// # │   ├── picture1.jpg -> this will come after compress file from soruce_dir
// # │   ├── picture2.jpg -> this will come after compress file from soruce_dir
// # ├── source_dir
// # │   ├── picture1.png
// # │   ├── picture2.gif
// # ├── src
// # │   ├── main.rs
// # ├── Cargo.lock
// # ├── Cargo.toml

//use 
//cargo run source_dir/ dest_dir/ 

fn main() {
    let source = PathBuf::from("source_dir");
    let destination = PathBuf::from("dest_dir");
    let quality = 80;
    
    compress_folder(&source, &destination, quality);
}



