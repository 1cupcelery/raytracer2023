use crate::ray::Ray;
use crate::vec3::Point3;
use crate::Vec3;
use std::mem::swap;

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Aabb {
    pub minimum: Point3,
    pub maximum: Point3,
}

impl Aabb {
    pub fn new(a: &Point3, b: &Point3) -> Self {
        Self {
            minimum: *a,
            maximum: *b,
        }
    }

    pub fn hit(&self, r: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.dir[a];
            let mut t0 = (self.minimum[a] - r.orig[a]) * inv_d;
            let mut t1 = (self.maximum[a] - r.orig[a]) * inv_d;
            if inv_d < 0.0 {
                swap(&mut t0, &mut t1);
            }
            t_min = t0.max(t_min);
            t_max = t1.min(t_max);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}

pub fn surrounding_box(box0: Aabb, box1: Aabb) -> Aabb {
    let small = Vec3::new(
        box0.minimum.x.min(box1.minimum.x),
        box0.minimum.y.min(box1.minimum.y),
        box0.minimum.z.min(box1.minimum.z),
    );
    let big = Vec3::new(
        box0.maximum.x.max(box1.maximum.x),
        box0.maximum.y.max(box1.maximum.y),
        box0.maximum.z.max(box1.maximum.z),
    );
    Aabb::new(&small, &big)
}
