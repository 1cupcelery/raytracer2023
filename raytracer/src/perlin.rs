use crate::rtweekend::random_usize_range;
use crate::vec3::Point3;
use crate::Vec3;
use std::ops::Mul;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    ranvec: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Self {
        let mut ranvec = Vec::new();
        for _i in 0..POINT_COUNT {
            ranvec.push(Vec3::random_range(-1.0, 1.0).unit_vector());
        }
        Self {
            ranvec,
            perm_x: Perlin::perlin_generate_perm(),
            perm_y: Perlin::perlin_generate_perm(),
            perm_z: Perlin::perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();
        let i = p.x().floor();
        let j = p.y().floor();
        let k = p.z().floor();
        let mut c: Vec<Vec<Vec<Vec3>>> = vec![
            vec![
                vec![Vec3::zero(), Vec3::zero()],
                vec![Vec3::zero(), Vec3::zero()],
            ],
            vec![
                vec![Vec3::zero(), Vec3::zero()],
                vec![Vec3::zero(), Vec3::zero()],
            ],
        ];
        for (di, item1) in c.iter_mut().enumerate().take(2) {
            for (dj, item2) in item1.iter_mut().enumerate().take(2) {
                for (dk, item3) in item2.iter_mut().enumerate().take(2) {
                    *item3 = self.ranvec[self.perm_x[((i + di as f64) as i32 & 255) as usize]
                        ^ self.perm_y[((j + dj as f64) as i32 & 255) as usize]
                        ^ self.perm_z[((k + dk as f64) as i32 & 255) as usize]];
                }
            }
        }
        Self::perlin_interp(&c, u, v, w)
    }

    fn perlin_interp(c: &[Vec<Vec<Vec3>>], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        for (i, item1) in c.iter().enumerate().take(2) {
            for (j, item2) in item1.iter().enumerate().take(2) {
                for (k, _item3) in item2.iter().enumerate().take(2) {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += c[i][j][k]
                        .mul(
                            (i as f64 * uu + (1 - i) as f64 * (1.0 - uu))
                                * (j as f64 * vv + (1 - j) as f64 * (1.0 - vv))
                                * (k as f64 * ww + (1 - k) as f64 * (1.0 - ww)),
                        )
                        .dot(weight_v);
                }
            }
        }
        accum
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

    pub fn turb(&self, p: &Point3, depth: usize) -> f64 {
        let mut accum = 0.0;
        let mut tmp_p = *p;
        let mut weight = 1.0;
        for _i in 0..depth {
            accum += weight * self.noise(&tmp_p);
            weight *= 0.5;
            tmp_p *= 2.0;
        } //depth=7
        accum.abs()
    }
}
