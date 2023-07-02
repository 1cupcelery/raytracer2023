use crate::aabb::Aabb;
use crate::material::Material;
use crate::ray::Ray;
use crate::rtweekend::degrees_to_radians;
use crate::vec3::Point3;
use crate::vec3::Vec3;
use std::ops::Sub;
pub use std::sync::Arc;

const INFINITY: f64 = f64::INFINITY;

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
            p: p1,
            normal: n1,
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

pub struct Translate {
    ptr: Arc<dyn Hittable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(p: Arc<dyn Hittable>, displacement: Vec3) -> Self {
        Self {
            ptr: p,
            offset: displacement,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r = Ray::new(&(ray.orig - self.offset), &ray.dir, ray.tm);
        if let Some(mut rec) = self.ptr.hit(&moved_r, t_min, t_max) {
            let n = rec.normal;
            rec.p += self.offset;
            rec.set_face_normal(&moved_r, &n);
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        if let Some(mut output_box) = self.ptr.bounding_box(time0, time1) {
            output_box = Aabb::new(
                &(output_box.minimum + self.offset),
                &(output_box.maximum + self.offset),
            );
            Some(output_box)
        } else {
            None
        }
    }
}

pub struct RotateY {
    ptr: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Option<Aabb>,
}

impl RotateY {
    pub fn new(p: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut bbox = p.bounding_box(0.0, 1.0);
        if let Some(b) = bbox {
            let mut max0 = Point3::new(INFINITY, INFINITY, INFINITY);
            let mut min0 = Point3::new(-INFINITY, -INFINITY, -INFINITY);
            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x = i as f64 * b.maximum.x + (1 - i) as f64 * b.minimum.x;
                        let y = j as f64 * b.maximum.y + (1 - j) as f64 * b.minimum.x;
                        let z = k as f64 * b.maximum.z + (1 - k) as f64 * b.minimum.x;

                        let newx = cos_theta * x + sin_theta * z;
                        let newz = -sin_theta * x + cos_theta * z;

                        let tester = Vec3::new(newx, y, newz);

                        for c in 0..3 {
                            min0[c] = min0[c].min(tester[c]);
                            max0[c] = max0[c].max(tester[c]);
                        }
                    }
                }
            }
            bbox = Some(Aabb::new(&min0, &max0));
        }
        Self {
            ptr: p,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = ray.orig;
        let mut direction = ray.dir;

        origin[0] = self.cos_theta * ray.orig[0] - self.sin_theta * ray.orig[2];
        origin[2] = self.sin_theta * ray.orig[0] + self.cos_theta * ray.orig[2];

        direction[0] = self.cos_theta * ray.dir[0] - self.sin_theta * ray.dir[2];
        direction[2] = self.sin_theta * ray.dir[0] + self.cos_theta * ray.dir[2];

        let rotated_r = Ray::new(&origin, &direction, ray.tm);

        if let Some(mut rec) = self.ptr.hit(&rotated_r, t_min, t_max) {
            let mut p = rec.p;
            let mut normal = rec.normal;

            p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
            p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

            normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
            normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

            rec.p = p;
            rec.set_face_normal(&rotated_r, &normal);

            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        self.bbox
    }
}
