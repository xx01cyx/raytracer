use crate::vec3::{Color, Point3};
use std::sync::Arc;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}

// Solid color

pub struct SolidColor {
    pub color_value: Color,
}

impl SolidColor {
    pub fn new(c: Color) -> Self {
        Self { color_value: c }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        self.color_value
    }
}

// Checker texture

pub struct CheckerTexture {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(c1: Color, c2: Color) -> Self {
        Self {
            even: Arc::new(SolidColor::new(c1)),
            odd: Arc::new(SolidColor::new(c2)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();

        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
