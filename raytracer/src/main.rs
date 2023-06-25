mod color;
mod hittable;
mod ray;
mod sphere;
mod vec3;

use crate::ray::Ray;
use crate::vec3::Color;
use crate::vec3::Point3;
use color::write_color;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::fs::File;
use std::ops::Mul;
pub use vec3::Vec3;

const AUTHOR: &str = "Celery";

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc: Vec3 = r.orig - *center;
    let a = r.dir.length_squared();
    let half_b = oc.dot(r.dir);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    return if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    };
}

fn ray_color(r: &Ray) -> Color {
    let mut t = hit_sphere(
        &Point3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        0.5,
        r,
    );
    if t > 0.0 {
        let N: Vec3 = (r.at(t)
            - Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            })
        .unit_vector();
        return Color {
            x: N.x + 1.0,
            y: N.y + 1.0,
            z: N.z + 1.0,
        }
        .mul(0.5);
    }
    let unit_direction: Vec3 = (&r.dir).unit_vector();
    t = 0.5 * (unit_direction.y + 1.0);
    Color {
        x: (1.0 - t) * 1.0 + t * 0.5,
        y: (1.0 - t) * 1.0 + t * 0.7,
        z: (1.0 - t) * 1.0 + t * 1.0,
    }
}

fn main() {
    // get environment variable CI, which is true for GitHub Actions
    let is_ci = is_ci();

    println!("CI: {}", is_ci);

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let height = (width as f64 / aspect_ratio) as usize;
    let path = "output/test.jpg";
    let quality = 60; // From 0 to 100, suggested value: 60

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let horizontal = Vec3 {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };
    let vertical = Vec3 {
        x: 0.0,
        y: viewport_height,
        z: 0.0,
    };
    let lower_left_corner = origin
        - horizontal / 2.0
        - vertical / 2.0
        - Vec3{
            x: 0.0,
            y: 0.0,
            z: focal_length,
        };

    // Create image data
    let mut img: RgbImage = ImageBuffer::new(width.try_into().unwrap(), height.try_into().unwrap());

    // Progress bar UI powered by library `indicatif`
    // You can use indicatif::ProgressStyle to make it more beautiful
    // You can also use indicatif::MultiProgress in multi-threading to show progress of each thread
    let bar = if is_ci {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };

    for j in 0..height {
        for i in 0..width {
            let u = i as f64 / (width - 1) as f64;
            let v = j as f64 / (height - 1) as f64;
            let r=Ray{
                orig: origin,
                dir: lower_left_corner + horizontal.mul(u) + vertical.mul(v) - origin,
            };
            let pixel_color = ray_color(&r);
            write_color(pixel_color, &mut img, i, height - j - 1);
            bar.inc(1);
        }
    }

    // Finish progress bar
    bar.finish();

    // Output image to file
    println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("Outputting image fails."),
    }
}