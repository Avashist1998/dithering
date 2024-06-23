use crate::utils::clamp;
use image::{ImageBuffer, Luma};


pub enum DitheringType {
    BlackAndWhite = 1,
    FloydSteinberg = 2,
    Atkinson = 3
}


pub fn floyd_steinberg_dithering(image: &mut ImageBuffer<Luma<u8>, Vec<u8>>, x: u32, y: u32) {

    let val: u8 = image.get_pixel(x, y)[0];
    let new_val: u8 = if val > 255 / 2 {225} else {0};
    let err: f32 = (((val as i16) - (new_val as i16)) as f32 / 255.0)/ 16.0;
    image.put_pixel(x, y, Luma([new_val]));

    if x+1 < image.width() {
        let pixel = image.get_pixel_mut(x+1, y);
        pixel[0] = clamp(pixel[0] as f32 + (7.0*err*255.0));

        if y+1 < image.height() {
            let pixel = image.get_pixel_mut(x+1, y+1);
            pixel[0] = clamp(pixel[0] as f32 + (1.0*err*255.0));
        }
    }
    if y+1 < image.height() {
        let pixel = image.get_pixel_mut(x, y+1);
        pixel[0] = clamp(pixel[0] as f32 + (5.0*err*255.0));

        if x > 0 {
            let pixel = image.get_pixel_mut(x-1, y+1);
            pixel[0] = clamp(pixel[0] as f32 + (3.0*err*255.0));
        } 
    }

}

pub fn atkinson_dithering(image: &mut ImageBuffer<Luma<u8>, Vec<u8>>, x: u32, y: u32) {

    let val: u8 = image.get_pixel(x, y)[0];
    let new_val: u8 = if val > 255 / 2 {225} else {0};
    let err: f32 = (((val as i16) - (new_val as i16)) as f32 / 255.0)/ 8.0;
    image.put_pixel(x, y, Luma([new_val]));

    if x+2 < image.width() {
        let pixel = image.get_pixel_mut(x+2, y);
        pixel[0] = clamp(pixel[0] as f32 + (1.0*err*255.0));
    }

    if x+1 < image.width() {
        let pixel = image.get_pixel_mut(x+1, y);
        pixel[0] = clamp(pixel[0] as f32 + (1.0*err*255.0));

        if y+1 < image.height() {
            let pixel = image.get_pixel_mut(x+1, y+1);
            pixel[0] = clamp(pixel[0] as f32 + (1.0*err*255.0));
        }
    }
    if y+1 < image.height() {
        let pixel = image.get_pixel_mut(x, y+1);
        pixel[0] = clamp(pixel[0] as f32 + (1.0*err*255.0));

        if x > 0 {
            let pixel = image.get_pixel_mut(x-1, y+1);
            pixel[0] = clamp(pixel[0] as f32 + (1.0*err*255.0));
        } 
    }

    if y+2 < image.height() {
        let pixel = image.get_pixel_mut(x, y+2);
        pixel[0] = clamp(pixel[0] as f32 + (1.0*err*255.0));
    }

}


pub fn black_and_white(image: &mut ImageBuffer<Luma<u8>, Vec<u8>>, x: u32, y: u32) {

    let pixel = image.get_pixel_mut(x, y);
    let new_val = if pixel[0] > 255 / 2 {225} else {0};
    pixel[0] = new_val;
}
