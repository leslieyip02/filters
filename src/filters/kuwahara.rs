use super::utils::*;
use image::{DynamicImage, GenericImageView, Pixel, Rgb, RgbImage};
use indicatif::ProgressIterator;

const WINDOW_SIZE: i32 = 3;
const QUADRANT_OFFSETS: [(i32, i32); 4] = [(-1, -1), (0, -1), (-1, 0), (0, 0)];

/// Returns the standard deviation within a quadrant
fn standard_deviation(pixels: &Vec<image::Rgb<u8>>, average: &image::Rgb<u8>) -> f64 {
    let mut total: u32 = 0;
    let squared_average = average.0.map(|value| (value as u32).pow(2));
    for pixel in pixels {
        pixel
            .0
            .iter()
            .enumerate()
            .for_each(|(i, &value)| total += (value as u32).pow(2).abs_diff(squared_average[i]));
    }
    (total as f64 / pixels.len() as f64).sqrt()
}

pub fn filter(reference: &DynamicImage) -> RgbImage {
    let (width, height) = reference.dimensions();
    let mut buffer = RgbImage::new(width, height);
    for (x0, y0, buffer_pixel) in buffer.enumerate_pixels_mut().progress() {
        let mut most_homogenous = f64::MAX;
        let mut resulting_pixel: Rgb<u8> = Rgb([0, 0, 0]);
        for (ox, oy) in QUADRANT_OFFSETS {
            let mut quadrant = Vec::<Rgb<u8>>::new();
            for i in 0..WINDOW_SIZE {
                let x1 = x0 as i32 + (ox * WINDOW_SIZE) + i;
                if x1 < 0 || x1 >= width as i32 {
                    continue;
                }
                for j in 0..WINDOW_SIZE {
                    let y1 = y0 as i32 + (oy * WINDOW_SIZE) + j;
                    if y1 < 0 || y1 >= height as i32 {
                        continue;
                    }
                    let quadrant_pixel = reference.get_pixel(x1 as u32, y1 as u32).to_rgb();
                    quadrant.push(quadrant_pixel);
                }
            }
            let average = average_rgb(&quadrant);
            let standard_deviation = standard_deviation(&quadrant, &average);
            if standard_deviation < most_homogenous {
                most_homogenous = standard_deviation;
                resulting_pixel = average;
            }
        }
        *buffer_pixel = resulting_pixel;
    }
    return buffer;
}
