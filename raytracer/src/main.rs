mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::ray::Ray;
use crate::rtweekend::{random_f64, random_f64_range};
use crate::sphere::Sphere;
use crate::vec3::Color;
use crate::vec3::Point3;
use color::write_color;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use material::Material;
use std::fs::File;
use std::ops::{Mul, Sub};
use std::sync::Arc;
pub use vec3::Vec3;

const AUTHOR: &str = "Celery";
const INFINITY: f64 = f64::INFINITY;

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

fn ray_color(r: &Ray, world: &dyn Hittable, depth: u8) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth == 0 {
        return Color {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }
    if let Some(rec) = world.hit(r, 0.001, INFINITY) {
        if let Some((attenuation, scattered)) = rec.mat_ptr.scatter(r, &rec) {
            return attenuation.mul(ray_color(&scattered, world, depth - 1));
        }
        return Color {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }
    let unit_direction: Vec3 = r.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    Color {
        x: (1.0 - t) * 1.0 + t * 0.5,
        y: (1.0 - t) * 1.0 + t * 0.7,
        z: (1.0 - t) * 1.0 + t * 1.0,
    }
}

pub fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let ground_material = Arc::new(Lambertian::new(&Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));
    for a in -11..10 {
        for b in -11..10 {
            let choose_mat = random_f64();
            let center = Point3::new(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );
            if center.sub(Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material>;
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    sphere_material = Arc::new(Lambertian::new(&albedo));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material.clone())));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_f64_range(0.0, 0.5);
                    sphere_material = Arc::new(Metal::new(&albedo, &fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material.clone())));
                } else {
                    sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material.clone())));
                }
            }
        }
    }
    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(&Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(&Color::new(0.7, 0.6, 0.5), &0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));
    world
}

fn main() {
    // get environment variable CI, which is true for GitHub Actions
    let is_ci = is_ci();

    println!("CI: {}", is_ci);

    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let path = "output/test.jpg";
    let quality = 60; // From 0 to 100, suggested value: 60
    let samples_per_pixel: u8 = 50;
    let max_depth: u8 = 500;

    // World
    let world = random_scene();

    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    // Create image data
    let mut img: RgbImage = ImageBuffer::new(
        image_width.try_into().unwrap(),
        image_height.try_into().unwrap(),
    );

    // Progress bar UI powered by library `indicatif`
    // You can use indicatif::ProgressStyle to make it more beautiful
    // You can also use indicatif::MultiProgress in multi-threading to show progress of each thread
    let bar = if is_ci {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((image_height * image_width) as u64)
    };

    for j in 0..image_height {
        for i in 0..image_width {
            let mut pixel_color = Color {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            for _s in 0..samples_per_pixel {
                let u = (i as f64 + random_f64()) / (image_width as f64 - 1.0);
                let v = (j as f64 + random_f64()) / (image_height as f64 - 1.0);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }
            write_color(
                pixel_color,
                &mut img,
                i,
                image_height - j - 1,
                samples_per_pixel,
            );
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
