use super::utils::weighted_average_rgb;
use image::{DynamicImage, GenericImageView, Pixel, Rgb, RgbImage};
use indicatif::ProgressIterator;
use std::f64::consts::PI;

const WINDOW_SIZE: i32 = 3;
const SIGMA: f64 = 0.5;
const AMOUNT: f64 = 1.0;

pub fn unsharp_masking(reference: &DynamicImage) -> RgbImage {
    let ss = SIGMA * SIGMA as f64;
    let k = 1.0 / (2.0 * PI * ss);
    let weights = (-WINDOW_SIZE..=WINDOW_SIZE)
        .map(|y| {
            (-WINDOW_SIZE..=WINDOW_SIZE)
                .map(move |x| k * (-(x * x + y * y) as f64 / (2.0 * ss)).exp())
        })
        .flat_map(|row| row)
        .collect::<Vec<f64>>();

    let (width, height) = reference.dimensions();
    let mut buffer = RgbImage::new(width, height);
    for (x0, y0, buffer_pixel) in buffer.enumerate_pixels_mut().progress() {
        let mut pixels: Vec<Rgb<u8>> = Vec::new();
        for ox in -WINDOW_SIZE..WINDOW_SIZE {
            let x1 = x0 as i32 + ox;
            if x1 < 0 || x1 >= width as i32 {
                continue;
            }

            for oy in -WINDOW_SIZE..WINDOW_SIZE {
                let y1 = y0 as i32 + oy;
                if y1 < 0 || y1 >= height as i32 {
                    continue;
                }

                let pixel = reference.get_pixel(x1 as u32, y1 as u32).to_rgb();
                pixels.push(pixel);
            }
        }
        let blurred = weighted_average_rgb(&pixels, &weights);
        let sharpened: [u8; 3] = reference
            .get_pixel(x0, y0)
            .to_rgb()
            .0
            .iter()
            .enumerate()
            .map(|(i, &value)| (value as f64 + (value as f64 - blurred.0[i] as f64) * AMOUNT) as u8)
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        *buffer_pixel = Rgb(sharpened);
    }
    return buffer;
}
