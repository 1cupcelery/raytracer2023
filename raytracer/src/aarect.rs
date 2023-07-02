use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Point3;
use crate::Vec3;
use std::sync::Arc;

pub struct XyRect {
    mp: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
}

impl XyRect {
    pub fn new(_x0: f64, _x1: f64, _y0: f64, _y1: f64, _k: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            x0: _x0,
            x1: _x1,
            y0: _y0,
            y1: _y1,
            k: _k,
            mp: mat,
        }
    }
}

impl Hittable for XyRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.orig.z) / ray.dir.z;
        if t < t_min || t > t_max {
            return None;
        }
        let x = ray.orig.x + t * ray.dir.x;
        let y = ray.orig.y + t * ray.dir.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        let mut rec = HitRecord::new(ray.at(t), outward_normal, self.mp.clone(), t);
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.set_face_normal(ray, &outward_normal);
        Some(rec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        let output_box = Aabb::new(
            &Point3::new(self.x0, self.y0, self.k - 0.0001),
            &Point3::new(self.x1, self.y1, self.k + 0.0001),
        );
        Some(output_box)
    }
}

pub struct XzRect {
    mp: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl XzRect {
    pub fn new(_x0: f64, _x1: f64, _z0: f64, _z1: f64, _k: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            x0: _x0,
            x1: _x1,
            z0: _z0,
            z1: _z1,
            k: _k,
            mp: mat,
        }
    }
}

impl Hittable for XzRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.orig.y) / ray.dir.y;
        if t < t_min || t > t_max {
            return None;
        }
        let x = ray.orig.x + t * ray.dir.x;
        let z = ray.orig.z + t * ray.dir.z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        let mut rec = HitRecord::new(ray.at(t), outward_normal, self.mp.clone(), t);
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.set_face_normal(ray, &outward_normal);
        Some(rec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        let output_box = Aabb::new(
            &Point3::new(self.x0, self.k - 0.0001, self.z0),
            &Point3::new(self.x1, self.k + 0.0001, self.z1),
        );
        Some(output_box)
    }
}

pub struct YzRect {
    mp: Arc<dyn Material>,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl YzRect {
    pub fn new(_y0: f64, _y1: f64, _z0: f64, _z1: f64, _k: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            y0: _y0,
            y1: _y1,
            z0: _z0,
            z1: _z1,
            k: _k,
            mp: mat,
        }
    }
}

impl Hittable for YzRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.orig.x) / ray.dir.x;
        if t < t_min || t > t_max {
            return None;
        }
        let y = ray.orig.y + t * ray.dir.y;
        let z = ray.orig.z + t * ray.dir.z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        let mut rec = HitRecord::new(ray.at(t), outward_normal, self.mp.clone(), t);
        rec.u = (y - self.y0) / (self.y1 - self.y0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.set_face_normal(ray, &outward_normal);
        Some(rec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        let output_box = Aabb::new(
            &Point3::new(self.k - 0.0001, self.y0, self.z0),
            &Point3::new(self.k + 0.0001, self.y1, self.z1),
        );
        Some(output_box)
    }
}
