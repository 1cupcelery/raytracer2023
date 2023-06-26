use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::rtweekend::random_f64;
use crate::vec3::Color;
use crate::Vec3;
use std::ops::{Mul, Sub};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(c: &Color) -> Self {
        Self {
            albedo: Color {
                x: c.x,
                y: c.y,
                z: c.z,
            },
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(&rec.p, &scatter_direction);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(c: &Color, f: &f64) -> Self {
        let f0 = if *f < 1.0 { *f } else { 1.0 };
        Self {
            albedo: Color {
                x: c.x,
                y: c.y,
                z: c.z,
            },
            fuzz: f0,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = Vec3::reflect(&r_in.dir.unit_vector(), &rec.normal);
        let scattered = Ray::new(
            &rec.p,
            &(reflected + Vec3::random_in_unit_sphere().mul(self.fuzz)),
        );
        let attenuation = self.albedo;
        if scattered.dir.dot(rec.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            ir: index_of_refraction,
        }
    }

    pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio: f64;
        if rec.front_face {
            refraction_ratio = 1.0 / self.ir;
        } else {
            refraction_ratio = self.ir;
        }
        let unit_direction = r_in.dir.unit_vector();
        let cos_theta: f64;
        if Vec3::zero().sub(unit_direction).dot(rec.normal) < 1.0 {
            cos_theta = Vec3::zero().sub(unit_direction).dot(rec.normal);
        } else {
            cos_theta = 1.0;
        }
        let sin_theta: f64 = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction: Vec3;
        if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random_f64() {
            direction = Vec3::reflect(&unit_direction, &rec.normal);
        } else {
            direction = Vec3::refract(&unit_direction, &rec.normal, refraction_ratio);
        }
        let scattered = Ray::new(&rec.p, &direction);
        Some((attenuation, scattered))
    }
}
