mod aabb;
mod aarect;
mod box_object;
mod bvh;
mod camera;
mod color;
mod constant_medium;
mod hittable;
mod hittable_list;
mod material;
mod moving_sphere;
mod perlin;
mod ray;
mod rtweekend;
mod sphere;
mod texture;
mod vec3;

use crate::aarect::{XyRect, XzRect, YzRect};
use crate::box_object::BoxObject;
use crate::bvh::BvhNode;
use crate::camera::Camera;
use crate::constant_medium::ConstantMedium;
use crate::hittable::{Hittable, RotateY, Translate};
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, DiffuseLight, Lambertian, Metal};
use crate::moving_sphere::MovingSphere;
use crate::ray::Ray;
use crate::rtweekend::{random_f64, random_f64_range};
use crate::sphere::Sphere;
use crate::texture::NoiseTexture;
use crate::texture::{CheckerTexture, ImageTexture};
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

fn ray_color(r: &Ray, background: &Color, world: &dyn Hittable, depth: u8) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth == 0 {
        return Color::zero();
    }
    // If the ray hits nothing, return the background color.
    if let Some(rec) = world.hit(r, 0.001, INFINITY) {
        let emitted = rec.mat_ptr.emitted(rec.u, rec.v, &rec.p);
        if let Some((attenuation, scattered)) = rec.mat_ptr.scatter(r, &rec) {
            emitted + attenuation.mul(ray_color(&scattered, background, world, depth - 1))
        } else {
            emitted
        }
    } else {
        *background
    }
}

pub fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let checker = Arc::new(CheckerTexture::new_color(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(checker)),
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
                    sphere_material = Arc::new(Lambertian::new_color(&albedo));
                    let center2 = center + Vec3::new(0.0, random_f64_range(0.0, 0.5), 0.0);
                    world.add(Arc::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material.clone(),
                    )));
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

    let material2 = Arc::new(Lambertian::new_color(&Color::new(0.4, 0.2, 0.1)));
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

fn two_spheres() -> HittableList {
    let mut objects = HittableList::new();
    let checker = Arc::new(CheckerTexture::new_color(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(checker.clone())),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(checker)),
    )));
    objects
}

pub fn two_perlin_spheres() -> HittableList {
    let mut objects = HittableList::new();
    let pertext = Arc::new(NoiseTexture::new(4.0));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(pertext.clone())),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(pertext)),
    )));
    objects
}

pub fn earth() -> HittableList {
    let mut objects = HittableList::new();
    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::new(earth_texture));
    let globe = Arc::new(Sphere::new(Point3::zero(), 2.0, earth_surface));
    objects.add(globe);
    objects
}

pub fn simple_light() -> HittableList {
    let mut objects = HittableList::new();
    let pertext = Arc::new(NoiseTexture::new(4.0));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(pertext.clone())),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(pertext)),
    )));
    let difflight = Arc::new(DiffuseLight::new_color(Color::new(4.0, 4.0, 4.0)));
    objects.add(Arc::new(XyRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight)));
    objects
}

pub fn cornell_box() -> HittableList {
    let mut objects = HittableList::new();
    let red = Arc::new(Lambertian::new_color(&Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new_color(&Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new_color(&Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_color(Color::new(15.0, 15.0, 15.0)));
    objects.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(Arc::new(XzRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )));
    objects.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    objects.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.add(Arc::new(XyRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    let box1_0 = Arc::new(BoxObject::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let box1_1 = Arc::new(RotateY::new(box1_0, 15.0));
    let box1_2 = Arc::new(Translate::new(box1_1, Vec3::new(265.0, 0.0, 295.0)));
    objects.add(box1_2);

    let box2_0 = Arc::new(BoxObject::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white,
    ));
    let box2_1 = Arc::new(RotateY::new(box2_0, -18.0));
    let box2_2 = Arc::new(Translate::new(box2_1, Vec3::new(130.0, 0.0, 65.0)));
    objects.add(box2_2);
    objects
}

pub fn cornell_smoke() -> HittableList {
    let mut objects = HittableList::new();
    let red = Arc::new(Lambertian::new_color(&Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new_color(&Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new_color(&Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_color(Color::new(7.0, 7.0, 7.0)));
    objects.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(Arc::new(XzRect::new(
        113.0, 443.0, 127.0, 432.0, 554.0, light,
    )));
    objects.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    objects.add(Arc::new(XyRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    let box1_0 = Arc::new(BoxObject::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let box1_1 = Arc::new(RotateY::new(box1_0, 15.0));
    let box1_2 = Arc::new(Translate::new(box1_1, Vec3::new(265.0, 0.0, 295.0)));

    let box2_0 = Arc::new(BoxObject::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white,
    ));
    let box2_1 = Arc::new(RotateY::new(box2_0, -18.0));
    let box2_2 = Arc::new(Translate::new(box2_1, Vec3::new(130.0, 0.0, 65.0)));

    objects.add(Arc::new(ConstantMedium::new_color(
        box1_2,
        0.01,
        Color::zero(),
    )));
    objects.add(Arc::new(ConstantMedium::new_color(
        box2_2,
        0.01,
        Color::new(1.0, 1.0, 1.0),
    )));
    objects
}

fn main() {
    // get environment variable CI, which is true for GitHub Actions
    let is_ci = is_ci();

    println!("CI: {}", is_ci);

    // Image
    let mut aspect_ratio = 16.0 / 9.0;
    let mut image_width = 400;
    let path = "output/test.jpg";
    let quality = 60; // From 0 to 100, suggested value: 60
    let mut samples_per_pixel: usize = 100;
    let max_depth: u8 = 50;

    // World
    // let world = random_scene();
    // let bvh = BvhNode::new_list(world, 0.0, 1.0);
    let mut world = HittableList::new();
    let mut lookfrom = Point3::new(13.0, 2.0, 3.0);
    let mut lookat = Point3::new(0.0, 0.0, 0.0);
    let mut vfov = 40.0;
    let mut aperture = 0.0;
    let mut background = Color::zero();

    let case = 7;
    match case {
        1 => {
            world = random_scene();
            background = Color::new(0.70, 0.80, 1.00);
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.1;
        }
        2 => {
            world = two_spheres();
            background = Color::new(0.70, 0.80, 1.00);
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        3 => {
            world = two_perlin_spheres();
            background = Color::new(0.70, 0.80, 1.00);
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        4 => {
            world = earth();
            background = Color::new(0.70, 0.80, 1.00);
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        5 => {
            world = simple_light();
            samples_per_pixel = 400;
            background = Color::zero();
            lookfrom = Point3::new(26.0, 3.0, 6.0);
            lookat = Point3::new(0.0, 2.0, 0.0);
            vfov = 20.0;
        }
        6 => {
            world = cornell_box();
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 200;
            background = Color::zero();
            lookfrom = Point3::new(278.0, 278.0, -800.0);
            lookat = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        7 => {
            world = cornell_smoke();
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 200;
            lookfrom = Point3::new(278.0, 278.0, -800.0);
            lookat = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        _ => {}
    }
    let bvh = BvhNode::new_list(world, 0.0, 1.0);

    // Camera
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
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
                //pixel_color += ray_color(&r, &world, max_depth);
                pixel_color += ray_color(&r, &background, &*bvh, max_depth);
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
