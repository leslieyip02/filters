use super::utils::average_squared_rgb;
use image::{DynamicImage, GenericImageView, Pixel, Rgb, RgbImage};
use indicatif::ProgressIterator;

const WINDOW_SIZE: i32 = 3;

pub fn box_blur(reference: &DynamicImage) -> RgbImage {
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
        *buffer_pixel = average_squared_rgb(&pixels);
    }
    return buffer;
}
