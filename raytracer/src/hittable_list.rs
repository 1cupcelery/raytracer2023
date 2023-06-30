use crate::aabb::{surrounding_box, Aabb};
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Point3;
pub use std::{sync::Arc, vec};

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec = Option::<HitRecord>::None;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            let a = object.hit(r, t_min, closest_so_far);
            if let Some(tmp) = a {
                closest_so_far = tmp.t;
                rec = Some(tmp);
            }
        }
        rec
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        if self.objects.is_empty() {
            return None;
        }
        let mut output_box = Aabb::new(&Point3::zero(), &Point3::zero());
        let mut first_box = true;
        for object in &self.objects {
            if let Some(tmp_box) = object.bounding_box(time0, time1) {
                output_box = if first_box {
                    tmp_box
                } else {
                    surrounding_box(output_box, tmp_box)
                };
                first_box = false;
            } else {
                return None;
            }
        }
        Some(output_box)
    }
}
