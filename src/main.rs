use filters::filters::kuwahara;
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
                    let result = kuwahara::filter(&image);
                    let destination = format!("./img/kuwahara/{}", filename);
                    result.save(destination).unwrap();
                }
            }
            None => (),
        }
    }
}
