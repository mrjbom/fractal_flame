use crate::fractal_info::transform::affine::AffineCoefs;
use euclid::Trig;
use nalgebra as na;
use nalgebra::{ComplexField, Vector2};
use rand::RngExt;
use rand::distr::Distribution;
use rand::distr::Uniform;
use rand::rngs::ChaCha12Rng;

/// r = sqrt(x^2 + y^2)
pub fn r(p: Vector2<f64>) -> f64 {
    (p.x * p.x + p.y * p.y).sqrt()
}

/// θ(theta) = arctan(x / y)
pub fn theta(p: Vector2<f64>) -> f64 {
    (p.x / p.y).atan()
}

/// φ(phi) = arctan(y / x)
pub fn phi(p: Vector2<f64>) -> f64 {
    (p.y / p.x).atan()
}

/// Ω(omega) = 0 or π
pub fn omega(rng: &mut ChaCha12Rng) -> f64 {
    if rng.random_bool(0.5) {
        0.0
    } else {
        std::f64::consts::PI
    }
}

/// Λ(lambda) = -1 or 1
pub fn lambda(rng: &mut ChaCha12Rng) -> f64 {
    if rng.random_bool(0.5) { -1.0 } else { 1.0 }
}

/// Ψ(psi) = random variable uniformly distributed on the interval [0, 1]
pub fn psi(psi_rng: &mut ChaCha12Rng, psi_rng_uniform: &Uniform<f64>) -> f64 {
    psi_rng_uniform.sample(psi_rng)
}

// --- VARIATIONS ---

// Linear (0)
pub fn linear(p: Vector2<f64>) -> Vector2<f64> {
    p
}

// Julia (13)
// √r · (cos(θ / 2 + Ω), sin(θ / 2 + Ω))
pub fn julia(p: Vector2<f64>, rng: &mut ChaCha12Rng) -> Vector2<f64> {
    let theta_div_2 = theta(p) / 2.0;
    let omega = omega(rng);
    let sqrt_r = r(p).sqrt();
    // cos(θ/2 + Ω)
    let x = (theta_div_2 + omega).cos();
    // sin(θ/2 + Ω)
    let y = (theta_div_2 + omega).sin();

    Vector2::new(sqrt_r * x, sqrt_r * y)
}
// Popcorn (17)
// (x + c * sin(tan(3 * y)), y + f * sin(tan(3 * x)))
pub fn popcorn(p: Vector2<f64>, affine_coefs: &AffineCoefs) -> Vector2<f64> {
    let c = affine_coefs.c;
    let f = affine_coefs.f;
    // sin(tan(3 * y))
    let sin1 = ((3.0 * p.y).tan()).sin();
    // sin(tan(3 * x))
    let sin2 = ((3.0 * p.x).tan()).sin();
    Vector2::new(p.x + c * sin1, p.y + f * sin2)
}

// PDJ (24)
// (sin(pdj_a * y) − cos(pdj_b * x), sin(pdj_c * x) − cos(pdj_d * y))
pub fn pdj(p: Vector2<f64>, pdj_params: PDJParams) -> Vector2<f64> {
    Vector2::new(
        (pdj_params.a * p.y).sin() - (pdj_params.b * p.x).cos(),
        (pdj_params.c * p.x).sin() - (pdj_params.d * p.y).cos(),
    )
}

#[derive(Copy, Clone, Debug)]
pub struct PDJParams {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
}
