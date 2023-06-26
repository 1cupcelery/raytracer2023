use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::ops::Mul;
use std::sync::Arc;

pub struct MovingSphere {
    pub center0: Point3,
    pub center1: Point3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        cen0: Point3,
        cen1: Point3,
        _time0: f64,
        _time1: f64,
        r: f64,
        m: Arc<dyn Material>,
    ) -> Self {
        Self {
            center0: cen0,
            center1: cen1,
            time0: _time0,
            time1: _time1,
            radius: r,
            mat_ptr: m,
        }
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = r.orig - self.center(r.tm);
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
        let n = (r.at(root) - self.center(r.tm)).unit_vector();
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let mut rec = HitRecord::new(r.at(root), n, self.mat_ptr.clone(), root);
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal: Vec3 = (rec.p - self.center(r.tm)) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat_ptr = self.mat_ptr.clone();
        Some(rec)
    }
}

impl MovingSphere {
    pub fn center(&self, time: f64) -> Point3 {
        self.center0
            + (self.center1 - self.center0).mul((time - self.time0) / (self.time1 - self.time0))
    }
}
