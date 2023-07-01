use crate::rtweekend::{random_f64, random_usize_range};
use crate::vec3::Point3;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    ranfloat: Vec<f64>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Self {
        let mut ranfloat = Vec::new();
        for _i in 0..POINT_COUNT {
            ranfloat.push(random_f64());
        }
        Self {
            ranfloat,
            perm_x: Perlin::perlin_generate_perm(),
            perm_y: Perlin::perlin_generate_perm(),
            perm_z: Perlin::perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let i = (((4.0 * p.x()) as i32) & 255) as usize;
        let j = (((4.0 * p.y()) as i32) & 255) as usize;
        let k = (((4.0 * p.z()) as i32) & 255) as usize;
        self.ranfloat[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }

    fn perlin_generate_perm() -> Vec<usize> {
        let mut p = vec![0; POINT_COUNT];
        for (i, item) in p.iter_mut().enumerate().take(POINT_COUNT) {
            *item = i;
        }
        Perlin::permute(&mut p, POINT_COUNT);
        p
    }

    fn permute(p: &mut Vec<usize>, n: usize) {
        for i in 0..n {
            let j = n - 1 - i;
            let target = random_usize_range(0, i);
            (*p).swap(j, target);
        }
    }
}
