# Rust Image Segmentation Using K-Means

This Rust program implements the k-means clustering algorithm to segment images by color. It utilizes parallel processing to enhance performance, making it suitable for handling large images efficiently.

## Features

- **Parallel Processing**: The assignment of pixels to centroids and the recalculating of centroids are parallelized using the Rayon library.
- **Image Processing**: Uses the `image` crate to handle image operations.
- **Command Line Interface**: Allows users to specify the input image and the number of desired clusters (k-values) directly via command line arguments.

## Dependencies

To run this project, you need Rust and Cargo installed on your machine. Additionally, the following crates are used:

- `rayon`: For parallel data processing.
- `ndarray`, `ndarray-rand`: For handling multidimensional data and random initialization of centroids.
- `image`: For reading and writing image files.
- `rand_distr`: For uniform distribution in random number generation.

Ensure you have these dependencies by adding them to your `Cargo.toml`:

```toml```

[dependencies]

rayon = "1.5.1"

ndarray = "0.15.3"

ndarray-rand = "0.14.0"

image = "0.23.14"

rand_distr = "0.4.2"


## Complie program
cargo build --release

## Run the program
cargo run --release -- <input_image_path> <k_value>

## Example:
cargo run --release -- images/tree.jpg 2

How It Works

Assign Pixels to Centroids: Each pixel's color is compared to all centroids to find the nearest centroid. This step is parallelized for efficiency.
    
Recalculate Centroids: New centroids are calculated by averaging the colors of all pixels assigned to each centroid, also parallelized.

Convergence Check: The algorithm iterates, adjusting centroids until changes between iterations are minimal, indicating stabilization.

Output

The output image will be saved in the same directory as the input image appended to the original file name. It visually represents the segmentation into different color clusters.
Contributions

