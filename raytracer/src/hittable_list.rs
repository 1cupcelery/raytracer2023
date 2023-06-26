pub use std::{sync::Arc, vec};
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

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

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec = Option::<HitRecord>::None;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            let a = object.hit(r, t_min, closest_so_far);
            if  let Some(tmp) = a {
                closest_so_far = tmp.t;
                rec = Some(tmp);
            }
        }
        rec
    }
}

