use crate::color::clamp;
use crate::perlin::Perlin;
use crate::vec3::Point3;
use crate::Color;
use std::ops::Mul;
use std::str;
use std::sync::Arc;

const BYTES_PER_PIXEL: usize = 3;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn new(c: Color) -> Self {
        Self { color_value: c }
    }

    // pub fn new_rgb(red: f64, green: f64, blue: f64) -> Self {
    //     Self {
    //         color_value: Color {
    //             x: red,
    //             y: green,
    //             z: blue,
    //         },
    //     }
    // }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.color_value
    }
}

#[derive(Clone)]
pub struct CheckerTexture {
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new_color(c1: Color, c2: Color) -> Self {
        Self {
            odd: Arc::new(SolidColor::new(c1)),
            even: Arc::new(SolidColor::new(c2)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(sc: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale: sc,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        Color::new(1.0, 1.0, 1.0)
            .mul(1.0 + (self.scale * p.z + 10.0 * self.noise.turb(p, 7)).sin())
            .mul(0.5)
    }
}

pub struct ImageTexture {
    data: Vec<u8>,
    width: usize,
    height: usize,
    bytes_per_scanline: usize,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        let img = image::open(filename)
            .expect("ERROR: Could not load texture image file.")
            .to_rgb8();
        let width = img.width();
        let height = img.height();
        let data = img.as_raw();
        Self {
            data: data.clone(),
            width: width as usize,
            height: height as usize,
            bytes_per_scanline: BYTES_PER_PIXEL * width as usize,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point3) -> Color {
        // If we have no texture data, then return solid cyan as a debugging aid.
        if self.data.is_empty() {
            return Color::new(0.0, 1.0, 1.0);
        }

        // Clamp input texture coordinates to [0,1] x [1,0]
        let u1 = clamp(u, 0.0, 1.0);
        let v1 = 1.0 - clamp(v, 0.0, 1.0); // Flip V to image coordinates

        let mut i = (u1 * self.width as f64) as usize;
        let mut j = (v1 * self.height as f64) as usize;

        // Clamp integer mapping, since actual coordinates should be less than 1.0
        if i >= self.width {
            i = self.width - 1;
        }
        if j >= self.height {
            j = self.height - 1;
        }

        let color_scale = 1.0 / 255.0;
        let index = j * self.bytes_per_scanline + i * BYTES_PER_PIXEL;

        Color::new(
            color_scale * self.data[index] as f64,
            color_scale * self.data[index + 1] as f64,
            color_scale * self.data[index + 2] as f64,
        )
    }
}
