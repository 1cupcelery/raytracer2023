use crate::vec3::Point3;
use crate::Color;
use std::sync::Arc;

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
    // pub fn new(_odd: Arc<dyn Texture>, _even: Arc<dyn Texture>) -> Self {
    //     Self {
    //         odd: _odd,
    //         even: _even,
    //     }
    // }

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
