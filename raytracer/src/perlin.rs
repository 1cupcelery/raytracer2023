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
        let mut u = p.x() - p.x().floor();
        let mut v = p.y() - p.y().floor();
        let mut w = p.z() - p.z().floor();
        u = u * u * (3.0 - 2.0 * u);
        v = v * v * (3.0 - 2.0 * v);
        w = w * w * (3.0 - 2.0 * w);
        let i = p.x().floor();
        let j = p.y().floor();
        let k = p.z().floor();
        let mut c: Vec<Vec<Vec<f64>>> = vec![
            vec![vec![0.0, 0.0], vec![0.0, 0.0]],
            vec![vec![0.0, 0.0], vec![0.0, 0.0]],
        ];
        for (di, item1) in c.iter_mut().enumerate().take(2) {
            for (dj, item2) in item1.iter_mut().enumerate().take(2) {
                for (dk, item3) in item2.iter_mut().enumerate().take(2) {
                    *item3 = self.ranfloat[self.perm_x[((i + di as f64) as i32 & 255) as usize]
                        ^ self.perm_y[((j + dj as f64) as i32 & 255) as usize]
                        ^ self.perm_z[((k + dk as f64) as i32 & 255) as usize]];
                }
            }
        }
        Self::trilinear_interp(&c, u, v, w)
    }

    fn trilinear_interp(c: &[Vec<Vec<f64>>], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for (i, item1) in c.iter().enumerate().take(2) {
            for (j, item2) in item1.iter().enumerate().take(2) {
                for (k, _item3) in item2.iter().enumerate().take(2) {
                    accum += (i as f64 * u + (1 - i) as f64 * (1.0 - u))
                        * (j as f64 * v + (1 - j) as f64 * (1.0 - v))
                        * (k as f64 * w + (1 - k) as f64 * (1.0 - w))
                        * c[i][j][k];
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
}
