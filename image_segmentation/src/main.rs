use rayon::prelude::*;
use ndarray::Array;
use ndarray::{ArrayBase, Axis, Dim, OwnedRepr};
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;
use std::sync::{Arc, Mutex};
use std::env;

use image::GenericImageView;

type Matrix = ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>;
type Cluster = Vec<Vec<usize>>;
type Centroids = Matrix;
type Features = Matrix;

// image segmentation using k-mean
// K-Means Algorithm code taken from
// https://applied-math-coding.medium.com/data-science-implementing-k-means-in-rust-457e4af55ece
// This function assigns each data point (in this case, a pixel represented by its color values in a feature space) to the nearest centroid. 
// The centroids are the current "average" colors that represent a cluster.
// Parallel Processing: It uses par_bridge() from Rayon for parallel iteration over the axis of the features array, x.
// Each pixel's color is compared to all centroids to find the nearest centroid.
// Error Calculation: It also calculates the sum of the squared Euclidean distance (error) from each pixel to its nearest centroid. 
// This helps in evaluating the performance of the clustering and deciding if the algorithm needs further iterations.

fn assign_to_centroids(centroids: &Centroids, x: &Arc<Features>) -> (Cluster, f64) {
    let mut cluster: Vec<Vec<usize>> = vec![vec![]; centroids.ncols()];
    let errors_and_indices: Vec<_> = x.axis_iter(Axis(0)).enumerate().par_bridge().map(|(i, row)| {
        let (closest_centroid_idx, err) = centroids
            .axis_iter(Axis(0))
            .map(|centroid| {
                let d = &centroid - &row;
                (&d * &d).sum()
            })
            .enumerate()
            .min_by(|(_, v1), (_, v2)| v1.total_cmp(v2))
            .unwrap();
        (closest_centroid_idx, i, err)
    }).collect();

    let mut total_err = 0.0;
    for (closest_centroid_idx, i, err) in errors_and_indices {
        cluster[closest_centroid_idx].push(i);
        total_err += err;
    }
    (cluster, total_err)
}
// Once pixels are assigned to clusters, this function recalculates the centroids by averaging the colors of all pixels assigned to each cluster.
// Parallel Processing: The outer loop iterating over each cluster is parallelized. 
// Inside this loop, the sum of the colors of all pixels in the cluster is computed (again, potentially in parallel), and the average is taken to update the centroid.

fn compute_centroids_from_cluster(cluster: &Cluster, x: &Arc<Features>) -> Centroids {
    let centroids = Mutex::new(Array::zeros((cluster.len(), x.ncols())));

    cluster.par_iter().enumerate().for_each(|(cluster_idx, row_indexes)| {
        let mut local_centroid = Array::zeros(x.ncols());
        if !row_indexes.is_empty() {
            let n = row_indexes.len() as f64;
            let sum: ArrayBase<OwnedRepr<f64>, Dim<[usize; 1]>> = row_indexes.iter()
                .map(|&row_idx| &x.row(row_idx) * (1.0 / n))
                .reduce(|a, b| a + b)
                .unwrap();
            local_centroid = sum;
        }
        let mut centroids = centroids.lock().unwrap();
        centroids.row_mut(cluster_idx).assign(&local_centroid);
    });

    let locked_centroids = centroids.lock().unwrap();
    locked_centroids.clone()
}



// Performs the k-means clustering algorithm.
// Set up centroids of segmentation
// Loop: For each iteration, it assigns pixels to the nearest centroids and then recalculates centroids based on these assignments.
// Convergence Check: The loop can exit early if changes in error between iterations are minimal, suggesting that the centroids have stabilized. 

fn k_means(k: usize, max_iter: usize, x: &Arc<Features>) -> (Centroids, Cluster) {
    let mut centroids = Array::random((k, x.ncols()), Uniform::new(0.0, 1.0));
    let mut cluster = vec![vec![]; k];
    let mut prev_total_err = f64::INFINITY;
    let total_err = 0.0;

    for iter in 0..max_iter {
        let (new_cluster, new_total_err) = assign_to_centroids(&centroids, &x);
        cluster = new_cluster;
        if iter > 0 && (prev_total_err - new_total_err).abs() / new_total_err < 0.01 {
            break;
        }
        prev_total_err = new_total_err;
        centroids = compute_centroids_from_cluster(&cluster, &x);
    }
    (centroids, cluster)
}

// cargo run --release -- <input_image> <k>
// cargo run --release -- images/tree.jpg 2

// Parses command line arguments and runs the k-means algorithm on an image.
fn main() {
    let (filename, seg_filename, k_val) = parse_args();
    let img = image::open(&filename).expect("Failed to open image");
    let (width, height) = img.dimensions();
    let mut ex = Array::zeros((height as usize * width as usize, 3));

    for (x, y, pixel) in img.pixels() {
        let i = (y * width + x) as usize; 
        ex[[i, 0]] = f64::from(pixel[0]);
        ex[[i, 1]] = f64::from(pixel[1]);
        ex[[i, 2]] = f64::from(pixel[2]);
    }

    let x = Arc::new(ex);
    let (centroids, cluster) = k_means(k_val, 100, &x);

    let mut new_img = image::ImageBuffer::new(width, height);
    for (cluster_idx, row_indexes) in cluster.iter().enumerate() {
        let pixel = image::Rgb([
            centroids[[cluster_idx, 0]] as u8,
            centroids[[cluster_idx, 1]] as u8,
            centroids[[cluster_idx, 2]] as u8,
        ]);
        for row_idx in row_indexes {
            let x = (*row_idx % width as usize) as u32;
            let y = (*row_idx / width as usize) as u32;
            new_img.put_pixel(x, y, pixel);
        }
    }
    new_img.save(&seg_filename).expect("Failed to save image");
}

/// Parses command line arguments to get the input and output file names and the number of centroids.
fn parse_args() -> (String, String, usize) {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Not enough arguments provided. Usage: <input_file> <k_value>");
    }
    let filename = args[1].clone();
    let seg_filename = filename.replace(".jpg", "_segmented.jpg");
    let k_val = args[2].parse().expect("k_value should be an integer");
    (filename, seg_filename, k_val)
}

// Assigning pixels to centroids (assign_to_centroids): This is done in parallel for all pixels, which speeds up the comparison with all centroids.
// Recalculating the centroids (compute_centroids_from_cluster): The update of each centroid based on its cluster's pixels is done in parallel, improving efficiency especially for large images with many pixels.
