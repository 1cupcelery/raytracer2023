use crate::aabb::Aabb;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Point3;
use crate::vec3::Vec3;
use std::f64::consts::PI;
pub use std::sync::Arc;

#[derive(Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(c: Point3, r: f64, m: Arc<dyn Material>) -> Self {
        Self {
            center: Point3 {
                x: c.x,
                y: c.y,
                z: c.z,
            },
            radius: r,
            mat_ptr: m,
        }
    }

    pub fn get_sphere_uv(p: &Point3) -> (f64, f64) {
        let theta = (-p.y).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;
        let u = phi / (2.0 * PI);
        let v = theta / PI;
        (u, v)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = r.orig - self.center;
        let a = r.dir.length_squared();
        let half_b = oc.dot(r.dir);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        let n = (r.at(root) - self.center).unit_vector();
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let mut rec = HitRecord::new(r.at(root), n, self.mat_ptr.clone(), root);
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        let (u1, v1) = Sphere::get_sphere_uv(&outward_normal);
        rec.u = u1;
        rec.v = v1;
        rec.mat_ptr = self.mat_ptr.clone();
        Some(rec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        let output_box = Aabb::new(
            &(self.center - Vec3::new(self.radius, self.radius, self.radius)),
            &(self.center + Vec3::new(self.radius, self.radius, self.radius)),
        );
        Some(output_box)
    }
}
