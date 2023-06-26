use crate::vec3::Vec3;
use crate::vec3::Point3;
use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use crate::ray::Ray;

#[derive(Clone, Debug, PartialEq)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(c: Point3, r: f64) -> Self {
        Self {
            center: Point3 {
                x: c.x,
                y: c.y,
                z: c.z,
            },
            radius: r,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = r.orig - self.center;
        let a = r.dir.length_squared();
        let half_b = oc.dot(r.dir);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0{
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
        let mut rec = HitRecord::new(r.at(root), n, root);
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal: Vec3  = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        Some(rec)
    }
}
