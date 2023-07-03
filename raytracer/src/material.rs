use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::rtweekend::random_f64;
use crate::texture::{SolidColor, Texture};
use crate::vec3::{Color, Point3};
use crate::Vec3;
use std::ops::{Mul, Sub};
use std::sync::Arc;

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color;
}

pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new_color(a: &Color) -> Self {
        Self {
            albedo: Arc::new(SolidColor::new(*a)),
        }
    }

    pub fn new(a: Arc<dyn Texture>) -> Self {
        Self { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(&rec.p, &scatter_direction, r_in.tm);
        let attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        Some((attenuation, scattered))
    }

    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::zero()
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
            r_in.tm,
        );
        let attenuation = self.albedo;
        if scattered.dir.dot(rec.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }

    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::zero()
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
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = r_in.dir.unit_vector();
        let cos_theta = if Vec3::zero().sub(unit_direction).dot(rec.normal) < 1.0 {
            Vec3::zero().sub(unit_direction).dot(rec.normal)
        } else {
            1.0
        };
        let sin_theta: f64 = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random_f64() {
                Vec3::reflect(&unit_direction, &rec.normal)
            } else {
                Vec3::refract(&unit_direction, &rec.normal, refraction_ratio)
            };
        let scattered = Ray::new(&rec.p, &direction, r_in.tm);
        Some((attenuation, scattered))
    }

    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::zero()
    }
}

pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new_color(c: Color) -> Self {
        Self {
            emit: Arc::new(SolidColor::new(c)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.emit.value(u, v, p)
    }
}

pub struct Isotropic {
    albedo: Arc<dyn Texture>,
}

impl Isotropic {
    // pub fn new_texture(a: Arc<dyn Texture>) -> Self {
    //     Self { albedo: a }
    // }

    pub fn new_color(c: Color) -> Self {
        Self {
            albedo: Arc::new(SolidColor::new(c)),
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        Some((
            self.albedo.value(rec.u, rec.v, &rec.p),
            Ray::new(&rec.p, &Vec3::random_in_unit_sphere(), r_in.tm),
        ))
    }

    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::zero()
    }
}
