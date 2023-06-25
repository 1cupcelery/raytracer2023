use crate::Vec3;
use crate::vec3::Point3;
use crate::ray;

struct hit_record {
    p: Point3,
    normal: Vec3,
    t: f64,
}
// class hittable {
// public:
// virtual bool hit(const ray& r, double t_min, double t_max, hit_record& rec) const = 0;
// };