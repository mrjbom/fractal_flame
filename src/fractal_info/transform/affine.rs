use nalgebra as na;
use nalgebra::Vector2;

#[derive(Clone)]
pub struct AffineCoefs {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub f: f64,
}

impl AffineCoefs {
    pub fn new(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> Self {
        Self { a, b, c, d, e, f }
    }
}

pub fn calculate_affine_transform(p: Vector2<f64>, affine_coefs: &AffineCoefs) -> Vector2<f64> {
    let x = p.x;
    let y = p.y;
    let AffineCoefs { a, b, c, d, e, f } = *affine_coefs;

    // (ax + by + c, dx + ey + f)
    Vector2::new(a * x + b * y + c, d * x + e * y + f)
}
