use image::RgbImage;
use crate::vec3::Color;

pub fn write_color(pixel_color: Color, img: &mut RgbImage, i: usize, j: usize) {
    let pixel = img.get_pixel_mut(i.try_into().unwrap(), j.try_into().unwrap());
    let colorx = pixel_color.x.sqrt();
    let colory = pixel_color.y.sqrt();
    let colorz = pixel_color.z.sqrt();
    // *pixel = image::Rgb([
    //     (within(0.0, 0.999, colorx) * 256.0) as u8,
    //     (within(0.0, 0.999, colory) * 256.0) as u8,
    //     (within(0.0, 0.999, colorz) * 256.0) as u8,
    // ]);
    *pixel = image::Rgb([
        (pixel_color.x * 255.999 as f64) as u8,
        (pixel_color.y * 255.999 as f64) as u8,
        (pixel_color.z * 255.999 as f64) as u8,
    ]);
    // Write the translated [0,255] value of each color component.
}
fn within(min: f64, max: f64, value: f64) -> f64 {
    if value > max {
        return max;
    }
    if value < min {
        return min;
    }
    value
}
