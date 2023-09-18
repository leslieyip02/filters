pub fn average_rgb(pixels: &Vec<image::Rgb<u8>>) -> image::Rgb<u8> {
    let mut rgb_sums = [0, 0, 0];
    for pixel in pixels {
        pixel
            .0
            .iter()
            .enumerate()
            .for_each(|(i, &value)| rgb_sums[i] += value as u32);
    }
    let n = pixels.len() as f64;
    let rgb_means = rgb_sums.map(|v| (v as f64 / n) as u8);
    image::Rgb(rgb_means)
}

pub fn average_squared_rgb(pixels: &Vec<image::Rgb<u8>>) -> image::Rgb<u8> {
    let mut rgb_sums = [0, 0, 0];
    for pixel in pixels {
        pixel
            .0
            .iter()
            .enumerate()
            .for_each(|(i, &value)| rgb_sums[i] += value.pow(2) as u32);
    }
    let n = pixels.len() as f64;
    let rgb_means = rgb_sums.map(|v| (v as f64 / n).sqrt() as u8);
    image::Rgb(rgb_means)
}
