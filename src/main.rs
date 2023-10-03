use filters::filters::*;
use std::{io, fs};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut line = String::new();

    println!("Filter: ");
    println!("1. blur ");
    println!("2. kuwahara ");
    println!("3. sepia ");
    println!("4. unsharp ");
    stdin.read_line(&mut line)?;
    let filter_index = line.trim().parse::<u8>().unwrap();
    line.clear();

    println!("Filename: ");
    stdin.read_line(&mut line)?;
    let filename = line.trim();
    let source = format!("./img/{}", filename);
    let image = image::open(source).unwrap();

    let result = match filter_index {
        1 => blur::gaussian_blur(&image),
        2 => kuwahara::normal_filter(&image, Some(8)),
        3 => sepia::sepia_filter(&image),
        _ => unsharp::unsharp_masking(&image)
    };  
    let destination = format!("./{}", filename);
    let _ = result.save(destination).unwrap();
    Ok(())

    // let paths = fs::read_dir("./img").unwrap();
    // for path in paths {
    //     let file = path.unwrap().path();
    //     match file.extension() {
    //         Some(extension) => {
    //             if extension == "jpg" || extension == "png" {
    //                 let filename = file.file_name().unwrap().to_str().unwrap();
    //                 println!("[Processing {}]", filename);
    //                 let source = format!("./img/{}", filename);
    //                 let image = image::open(source).unwrap();
    //                 let result = sepia::sepia_filter(&image);
    //                 let destination = format!("./img/sepia/{}", filename);
    //                 let _ = result.save(destination).unwrap();
    //             }
    //         }
    //         None => (),
    //     }
    // }
}
