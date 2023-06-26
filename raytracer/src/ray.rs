use crate::vec3::Point3;
use crate::vec3::Vec3;

#[derive(Clone, Debug, PartialEq)]

pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: &Point3, direction: &Vec3) -> Self {
        Self {
            orig: Point3 {
                x: origin.x,
                y: origin.y,
                z: origin.z,
            },
            dir: Vec3 {
                x: direction.x,
                y: direction.y,
                z: direction.z,
            },
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        Point3 {
            x: self.orig.x + t * self.dir.x,
            y: self.orig.y + t * self.dir.y,
            z: self.orig.z + t * self.dir.z,
        }
    }
}
