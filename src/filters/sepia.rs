use image::{DynamicImage, GenericImageView, Rgb, RgbImage, Pixel};
use indicatif::ProgressIterator;

const FACTORS: [[f64; 3]; 3] = [
    [ 0.393, 0.769, 0.189 ],
    [ 0.349, 0.686, 0.168 ],
    [ 0.272, 0.534, 0.131 ]
];

pub fn sepia_filter(reference: &DynamicImage) -> RgbImage {
    let (width, height) = reference.dimensions();
    let mut buffer = RgbImage::new(width, height);
    for (x0, y0, buffer_pixel) in buffer.enumerate_pixels_mut().progress() {
        let pixel = reference.get_pixel(x0, y0).to_rgb();
        let rgb_values = (0..3)
            .map(|i| {
                let row = &FACTORS[i];
                pixel
                    .0
                    .into_iter()
                    .enumerate()
                    .map(|(j, value)| row[j] * value as f64)
                    .sum::<f64>()
                    .clamp(0.0, 255.0) as u8
            })
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        *buffer_pixel = Rgb(rgb_values);
    }
    return buffer;
}