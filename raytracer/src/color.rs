use crate::vec3::Color;
use image::RgbImage;

pub fn write_color(
    pixel_color: Color,
    img: &mut RgbImage,
    i: usize,
    j: usize,
    samples_per_pixel: usize,
) {
    let pixel = img.get_pixel_mut(i.try_into().unwrap(), j.try_into().unwrap());
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;
    // Divide the color by the number of samples.
    let scale = 1.0 / samples_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();
    *pixel = image::Rgb([
        (champ(r, 0.0, 0.999) * 256.0) as u8,
        (champ(g, 0.0, 0.999) * 256.0) as u8,
        (champ(b, 0.0, 0.999) * 256.0) as u8,
    ]);
    // Write the translated [0,255] value of each color component.
}

pub fn champ(value: f64, min: f64, max: f64) -> f64 {
    if value > max {
        return max;
    }
    if value < min {
        return min;
    }
    value
}
