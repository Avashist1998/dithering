use wasm_bindgen::prelude::*;
use web_sys::console;


pub fn clamp(value: f32) -> u32 {
    if value < 0.0 {
        return 0;
    } else if value > 255.0 {
        return 255;
    } else {
        return value as u32;
    }
}

fn data_to_grayscale(data: &[u8]) -> Vec<u32> {
    let mut grayscale = Vec::new();

    for chunk in data.chunks(4) {
        // Assume data is in RGBA format
        let r = chunk[0] as u32;
        let g = chunk[1] as u32;
        let b = chunk[2] as u32;
        let alpha = chunk[3];

        // Compute grayscale value
        let gray = (r + g + b) / 3;
        grayscale.push(gray);
    }
    return grayscale;
}


fn get_index(i: u32, xShift: i32, yShift: i32, width: u32) -> i32 {
    let y: u32 = i%width;
    let x: u32 = i / width;
    return (width as i32)*(x as i32 + xShift) + (y as i32 + yShift)
}


fn floyd_steinberg_dithering_helper(i: u32, image: &mut Vec<u32>, width: u32) -> u32 {

    let threshold = 125;
    let val: u32 = image[i as usize];
    let new_val: u32 = if val > threshold {225} else {0};
    let err: f32 = (((val as i32) - (new_val as i32)) as f32 / 255.0)/ 16.0;
    image[i as usize] = new_val;

    let errorBlocks = [[1, 0, 7], [1, 1, 1], [0, 1, 5], [-1, 1, 3]];

    for block in errorBlocks {
        let index = get_index(i, block[0], block[1], width);
        if index >= 0 && (index as usize) < image.len() {
            image[index as usize] = clamp(image[index as usize] as f32 + ((block[2] as f32)*err*255.0));
        }
    }
    return image[i as usize]
}



fn atkinson_dithering_helper(i: u32, image: &mut Vec<u32>, width: u32) -> u32 {

    let threshold = 125;
    let val: u32 = image[i as usize];
    let new_val: u32 = if val > threshold {225} else {0};
    let err: f32 = (((val as i32) - (new_val as i32)) as f32 / 255.0)/ 16.0;
    image[i as usize] = new_val;

    let errorBlocks = [[2, 0, 1], [1, 0, 1], [1, 1, 1], [0, 1, 1], [-1, 1, 1], [0, 2, 1]];

    for block in errorBlocks {
        let index = get_index(i, block[0], block[1], width);
        if index >= 0 && (index as usize) < image.len() {
            image[index as usize] = clamp(image[index as usize] as f32 + ((block[2] as f32)*err*255.0));
        }
    }
    return image[i as usize]
}


#[wasm_bindgen]
pub fn black_and_white(data: &[u8], width: u32) -> Vec<u8> {
    // Example processing: Convert image to floyd and steinberg dithering
    let threshold = 125;
    let mut image = data_to_grayscale(data);
    let mut res_image = Vec::new();
    let mut i = 0;
    for chunk in data.chunks(4) {
        // Assume data is in RGBA format
        let alpha = chunk[3];
        let gray = image[i];
        // Apply threshold
        let binary_value = if gray > threshold { 255 } else { 0 };
        i += 1;
        res_image.push(binary_value);
        res_image.push(binary_value);
        res_image.push(binary_value);
        res_image.push(alpha); // Preserve alpha
    }

    console::log_1(&"Image processed".into());
    res_image
}

#[wasm_bindgen]
pub fn floyd_steinberg_dithering(data: &[u8], width: u32) -> Vec<u8> {
    // Example processing: Convert image to floyd and steinberg dithering
    let threshold = 125;
    let mut image = data_to_grayscale(data);
    let mut res_image = Vec::new();
    let mut i = 0;
    for chunk in data.chunks(4) {
        // Assume data is in RGBA format
        let alpha = chunk[3];
        let binary_value = floyd_steinberg_dithering_helper(i, &mut image, width) as u8;
        i += 1;
        res_image.push(binary_value);
        res_image.push(binary_value);
        res_image.push(binary_value);
        res_image.push(alpha); // Preserve alpha
    }

    console::log_1(&"Image processed".into());
    res_image
}



#[wasm_bindgen]
pub fn atkinson_dithering(data: &[u8], width: u32) -> Vec<u8> {
    // Example processing: Convert image to floyd and steinberg dithering
    let threshold = 125;
    let mut image = data_to_grayscale(data);
    let mut res_image = Vec::new();
    let mut i = 0;
    for chunk in data.chunks(4) {
        // Assume data is in RGBA format
        let alpha = chunk[3];
        let binary_value = atkinson_dithering_helper(i, &mut image, width) as u8;
        i += 1;
        res_image.push(binary_value);
        res_image.push(binary_value);
        res_image.push(binary_value);
        res_image.push(alpha); // Preserve alpha
    }

    console::log_1(&"Image processed".into());
    res_image
}




#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console::log_1(&"Wasm module initialized".into());
    Ok(())
}