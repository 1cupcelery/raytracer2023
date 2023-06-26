use crate::aabb::{surrounding_box, AABB};
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::rtweekend::random_usize_range;
use std::sync::Arc;

#[derive(Clone)]
pub struct BvhNode {
    pub left: Option<Arc<dyn Hittable>>,
    pub right: Option<Arc<dyn Hittable>>,
    pub box_0: AABB,
}

impl BvhNode {
    fn new(
        left: Option<Arc<dyn Hittable>>,
        right: Option<Arc<dyn Hittable>>,
        time0: f64,
        time1: f64,
    ) -> Self {
        if left.is_none() {
            panic!("BvhNode get null left child!");
        }
        let box0 = left.as_ref().unwrap().bounding_box(time0, time1).unwrap();
        if right.is_some() {
            let box1 = right.as_ref().unwrap().bounding_box(time0, time1).unwrap();
            Self {
                left,
                right,
                box_0: surrounding_box(box0, box1),
            }
        } else {
            Self {
                left,
                right,
                box_0: box0,
            }
        }
    }

    pub fn new_list(list: HittableList, time0: f64, time1: f64) -> Self {
        Self::new_vec(list.objects, time0, time1)
    }

    pub fn new_vec(mut objects: Vec<Arc<dyn Hittable>>, time0: f64, time1: f64) -> Self {
        let axis = random_usize_range(0, 2);
        let comparator=|a:&Arc<dyn Hittable>, b:&Arc<dyn Hittable>| {
            a.bounding_box(time0, time1).unwrap().minimum[axis]
                .partial_cmp(&b.bounding_box(time0, time1).unwrap().minimum[axis])
                .unwrap()
        };
        let object_span = objects.len();
        if object_span == 0 {
            panic!();
        } else if object_span == 1 {
            let l = objects.pop().unwrap();
            Self::new(Some(l), None, time0, time1)
        } else if object_span == 2 {
            objects.sort_by(comparator);
            let l = objects.pop().unwrap();
            let r = objects.pop().unwrap();
            Self::new(Some(l), Some(r), time0, time1)
        } else {
            objects.sort_by(comparator);
            let mut left_objects = objects;
            let right_objects = left_objects.split_off(left_objects.len() / 2);
            let l = Arc::new(Self::new_vec(left_objects, time0, time1));
            let r = Arc::new(Self::new_vec(right_objects, time0, time1));
            Self::new(Some(l), Some(r), time0, time1)
        }
    }
}


impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if self.box_0.hit(ray, t_min, t_max) {
            if let Some(hit_left) = self.left.clone().unwrap().hit(ray, t_min, t_max) {
                if self.right.is_none() {
                    return Some(hit_left);
                }
                return if let Some(hit_right) = self.right.clone().unwrap().hit(ray, t_min, t_max) {
                    if hit_left.t < hit_right.t {
                        Some(hit_left)
                    } else {
                        Some(hit_right)
                    }
                } else {
                    Some(hit_left)
                };
            } else {
                if self.right.is_none() {
                    return None;
                }
                if let Some(hit_right) = self.right.clone().unwrap().hit(ray, t_min, t_max) {
                    return Some(hit_right);
                } else {
                    None
                }
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(self.box_0)
    }
}
