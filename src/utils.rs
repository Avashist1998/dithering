pub fn clamp(value: f32) -> u8 {
    if value < 0.0 {
        return 0;
    } else if value > 255.0 {
        return 255;
    } else {
        return value as u8;
    }
}
