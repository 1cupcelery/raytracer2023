use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::material::{Isotropic, Material};
use crate::ray::Ray;
use crate::rtweekend::random_f64;
//use crate::texture::Texture;
use crate::vec3::Color;
use crate::Vec3;
use std::sync::Arc;

const INFINITY: f64 = f64::INFINITY;

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    phase_function: Arc<dyn Material>,
    neg_inv_density: f64,
}

impl ConstantMedium {
    // pub fn new_texture(b: Arc<dyn Hittable>, d: f64, a: Arc<dyn Texture>) -> Self {
    //     Self {
    //         boundary: b,
    //         phase_function: Arc::new(Isotropic::new_texture(a)),
    //         neg_inv_density: -1.0 / d,
    //     }
    // }

    pub fn new_color(b: Arc<dyn Hittable>, d: f64, c: Color) -> Self {
        Self {
            boundary: b,
            phase_function: Arc::new(Isotropic::new_color(c)),
            neg_inv_density: -1.0 / d,
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Print occasional samples when debugging. To enable, set enableDebug true.
        if let Some(mut rec1) = self.boundary.hit(ray, -INFINITY, INFINITY) {
            if let Some(mut rec2) = self.boundary.hit(ray, rec1.t + 0.0001, INFINITY) {
                if rec1.t < t_min {
                    rec1.t = t_min;
                }
                if rec2.t > t_max {
                    rec2.t = t_max;
                }
                if rec1.t >= rec2.t {
                    return None;
                }
                if rec1.t < 0.0 {
                    rec1.t = 0.0;
                }
                let ray_length = ray.dir.length();
                let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                let hit_distance = self.neg_inv_density * random_f64().log(2.0);

                if hit_distance > distance_inside_boundary {
                    return None;
                }
                let t1 = rec1.t + hit_distance / ray_length;
                let p1 = ray.at(t1);
                let n1 = Vec3::new(1.0, 0.0, 0.0); // arbitrary
                let m1 = self.phase_function.clone();
                let mut rec = HitRecord::new(p1, n1, m1, t1);
                rec.front_face = true; // also arbitrary
                Some(rec)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        self.boundary.bounding_box(time0, time1)
    }
}
