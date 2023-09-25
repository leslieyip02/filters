use filters::filters::*;
use std::fs;

fn main() {
    let paths = fs::read_dir("./img").unwrap();
    for path in paths {
        let file = path.unwrap().path();
        match file.extension() {
            Some(extension) => {
                if extension == "jpg" || extension == "png" {
                    let filename = file.file_name().unwrap().to_str().unwrap();
                    println!("[Processing {}]", filename);
                    let source = format!("./img/{}", filename);
                    let image = image::open(source).unwrap();
                    let result = blur::gaussian_blur(&image);
                    let destination = format!("./img/blur/gaussian/{}", filename);
                    let _ = result.save(destination).unwrap();
                }
            }
            None => (),
        }
    }
}
