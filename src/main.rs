mod dithering;
mod utils;

use clap::{Arg, Command};
use image::{open, DynamicImage, ImageBuffer, ImageError, Luma};
use dithering::{DitheringType, black_and_white, floyd_steinberg_dithering, atkinson_dithering};


fn generate_dithering_img(file_name: &str, kind: DitheringType) -> Result<i8, ImageError> {

    let img: DynamicImage = open(format!("images/{}", file_name))?;
    let mut img_data: ImageBuffer<Luma<u8>, Vec<u8>>    = img.to_luma8();

    println!(
        "Before: {}x{} {:?} ({} channels)",
        img.width(),
        img.height(),
        img.color(),
        img.color().channel_count()
    );

    for y in 0..img_data.height() {
            for x in 0..img_data.width() {
            match kind {
                DitheringType::BlackAndWhite => black_and_white(&mut img_data, x, y),
                DitheringType::FloydSteinberg => floyd_steinberg_dithering(&mut img_data, x, y),
                DitheringType::Atkinson => atkinson_dithering(&mut img_data, x, y)
            }
        }
    }

    match kind {
        DitheringType::BlackAndWhite => img_data.save(format!("res_images/b_a_w_{}", file_name))?,
        DitheringType::FloydSteinberg => img_data.save(format!("res_images/f_s_d_{}", file_name))?,
        DitheringType::Atkinson => img_data.save(format!("res_images/a_d_{}", file_name))?
    }
    return Ok(0);
}


fn main() {
    let matches = Command::new("Image Dithering")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Applies dithering to an image")
        .arg(
            Arg::new("file")
                .help("The image file to process")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("dithering")
                .help("The type of dithering to apply")
                .required(true)
                .index(2)
        )
        .get_matches();
    
    let default_filename = "david.jpg".to_string();
    let default_filter: String = "bw".to_string();
    let file_name = matches.get_one::<String>("file").unwrap_or(&default_filename);
    let dithering_type = matches.get_one::<String>("dithering").unwrap_or(&default_filter);

    let kind = match dithering_type.as_str() {
        "bw" => DitheringType::BlackAndWhite,
        "fs" => DitheringType::FloydSteinberg,
        "atkinson" => DitheringType::Atkinson,
        _ => unreachable!(),
    };

    if let Err(e) = generate_dithering_img(file_name, kind) {
        eprintln!("Error: {:?}", e);
    } else {
        println!("Dithering applied successfully.");
    }
}
