use crate::aabb::Aabb;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Point3;
use crate::vec3::Vec3;
use std::ops::Sub;
pub use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub mat_ptr: Arc<dyn Material>,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p1: Point3, n1: Vec3, m1: Arc<dyn Material>, t1: f64) -> Self {
        Self {
            p: Point3 {
                x: p1.x,
                y: p1.y,
                z: p1.z,
            },
            normal: Vec3 {
                x: n1.x,
                y: n1.y,
                z: n1.z,
            },
            t: t1,
            mat_ptr: m1,
            u: 0.0,
            v: 0.0,
            front_face: true,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        if r.dir.dot(*outward_normal) > 0.0 {
            // ray is inside the sphere
            self.normal = Vec3::zero().sub(*outward_normal);
            self.front_face = false;
        } else {
            // ray is outside the sphere
            self.normal = *outward_normal;
            self.front_face = true;
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;
}
