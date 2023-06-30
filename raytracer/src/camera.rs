use crate::ray::Ray;
use crate::rtweekend::{degrees_to_radians, random_f64_range};
use crate::vec3::{Point3, Vec3};
use std::ops::Mul;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    //w: Vec3,
    lens_radius: f64,
    time0: f64,
    time1: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        time0: f64,
        time1: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let w0 = (lookfrom - lookat).unit_vector();
        let u0 = vup.cross(w0).unit_vector();
        let v0 = w0.cross(u0);
        let horizontal0 = u0.mul(viewport_width).mul(focus_dist);
        let vertical0 = v0.mul(viewport_height).mul(focus_dist);
        let lower_left_corner0 =
            lookfrom - horizontal0 / 2.0 - vertical0 / 2.0 - w0.mul(focus_dist);
        Self {
            //w: w0,
            u: u0,
            v: v0,
            origin: lookfrom,
            horizontal: horizontal0,
            vertical: vertical0,
            lower_left_corner: lower_left_corner0,
            lens_radius: aperture / 2.0,
            time0,
            time1,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Vec3::random_in_unit_disk().mul(self.lens_radius);
        let offset = self.u.mul(rd.x) + self.v.mul(rd.y);
        Ray::new(
            &(self.origin + offset),
            &(self.lower_left_corner + self.horizontal.mul(s) + self.vertical.mul(t)
                - self.origin
                - offset),
            random_f64_range(self.time0, self.time1),
        )
    }
}
