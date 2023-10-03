use super::utils::*;
use image::{DynamicImage, GenericImageView, Pixel, Rgb, RgbImage};
use indicatif::ProgressIterator;
use std::collections::HashSet;
use std::f64::consts::PI;

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

const WINDOW_SIZE: i32 = 3;
const QUADRANT_OFFSETS: [(i32, i32); 4] = [(-1, -1), (0, -1), (-1, 0), (0, 0)];

pub fn normal_filter(reference: &DynamicImage, window_size: Option<i32>) -> RgbImage {
    let window_size = match window_size {
        Some(size) => size,
        None => WINDOW_SIZE
    };

    let (width, height) = reference.dimensions();
    let mut buffer = RgbImage::new(width, height);
    let cache_width = width as usize + window_size as usize + 1;
    let cache_height = height as usize + window_size as usize + 1;
    let mut averages = vec![vec![Rgb([0 as u8; 3]); cache_width]; cache_height];
    let mut standard_deviations = vec![vec![-1.0; cache_width]; cache_height];
    for (x0, y0, buffer_pixel) in buffer.enumerate_pixels_mut().progress() {
        let mut most_homogenous = f64::MAX;
        let mut resulting_pixel: Rgb<u8> = *buffer_pixel;
        for (ox, oy) in QUADRANT_OFFSETS {
            let x1 = (x0 as i32 + window_size + ox * window_size) as usize;
            let y1 = (y0 as i32 + window_size + oy * window_size) as usize;
            if standard_deviations[y1][x1] == -1.0 {
                let mut quadrant = Vec::<Rgb<u8>>::new();
                for i in 0..window_size {
                    let x2 = x0 as i32 + (ox * window_size) + i;
                    if x2 < 0 || x2 >= width as i32 {
                        continue;
                    }

                    for j in 0..window_size {
                        let y2 = y0 as i32 + (oy * window_size) + j;
                        if y2 < 0 || y2 >= height as i32 {
                            continue;
                        }

                        let quadrant_pixel = reference.get_pixel(x2 as u32, y2 as u32).to_rgb();
                        quadrant.push(quadrant_pixel);
                    }
                }
                let average = average_rgb(&quadrant);
                let standard_deviation = standard_deviation(&quadrant, &average);
                if standard_deviation < most_homogenous {
                    most_homogenous = standard_deviation;
                    resulting_pixel = average;
                }
                averages[y1][x1] = average;
                standard_deviations[y1][x1] = standard_deviation;
            }

            let average = averages[y1][x1];
            let standard_deviation = standard_deviations[y1][x1];
            if standard_deviation < most_homogenous {
                most_homogenous = standard_deviation;
                resulting_pixel = average;
            }
        }
        *buffer_pixel = resulting_pixel;
    }
    return buffer;
}

const RADIUS: f64 = 4.0;

pub fn generalized_filter(reference: &DynamicImage) -> RgbImage {
    let (width, height) = reference.dimensions();
    let mut buffer = RgbImage::new(width, height);
    for (x0, y0, buffer_pixel) in buffer.enumerate_pixels_mut().progress() {
        let x0 = x0 as f64 + 0.5;
        let y0 = y0 as f64 + 0.5;
        let mut sectors: [Vec<Rgb<u8>>; 8] = Default::default();
        let mut sector_cells: [HashSet<(i32, i32)>; 8] = Default::default();

        let lx = ((x0 - RADIUS) as i32).max(0);
        let ly = ((y0 - RADIUS) as i32).max(0);
        let rx = ((x0 + RADIUS) as i32).min(width as i32);
        let ry = ((y0 + RADIUS) as i32).min(height as i32);
        for i in ly..ry {
            let yy = (y0 - i as f64).powf(2.0);
            for j in lx..rx {
                let xx = (x0 - j as f64).powf(2.0);
                let distance = (xx + yy).sqrt();
                if distance > RADIUS {
                    continue;
                }

                let angle = (y0 - i as f64).atan2(x0 - j as f64) - (PI / 16.0);
                let k = (angle / (PI / 8.0)) as usize;
                for (ox, oy) in QUADRANT_OFFSETS {
                    let x1 = j + ox;
                    let y1 = i + oy;
                    if x1 < 0 || x1 >= width as i32 || y1 < 0 || y1 >= height as i32 {
                        continue;
                    }

                    let c = (x1, y1);
                    if !sector_cells[k].contains(&c) {
                        let sector_pixel = reference.get_pixel(x1 as u32, y1 as u32).to_rgb();
                        sectors[k].push(sector_pixel);
                        sector_cells[k].insert(c);
                    }
                    sector_cells.iter_mut().for_each(|cells| cells.clear());
                }
            }
        }

        let mut most_homogenous = f64::MAX;
        let mut resulting_pixel: Rgb<u8> = image::Rgb([255, 255, 255]);
        for sector in sectors {
            if !sector.is_empty() {
                let average = average_rgb(&sector);
                let standard_deviation = standard_deviation(&sector, &average);
                if standard_deviation < most_homogenous {
                    most_homogenous = standard_deviation;
                    resulting_pixel = average.to_owned();
                }
            }
        }        
        *buffer_pixel = resulting_pixel;
    }
    return buffer;
}
