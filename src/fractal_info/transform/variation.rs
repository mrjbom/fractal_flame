pub mod variations;

use crate::fractal_info::transform::affine::AffineCoefs;
use crate::fractal_info::transform::variation::variations::{
    PDJParams, bent, handkerchief, julia, pdj, popcorn,
};
use nalgebra as na;
use nalgebra::Vector2;
use rand::rngs::ChaCha12Rng;

#[derive(Clone, Debug)]
pub struct VariationAndWeight {
    pub variation: Variation,
    pub weight: f64,
}

impl VariationAndWeight {
    pub fn new(variation: Variation, weight: f64) -> Self {
        Self { variation, weight }
    }
}

impl From<(Variation, f64)> for VariationAndWeight {
    fn from(value: (Variation, f64)) -> Self {
        Self {
            variation: value.0,
            weight: value.1,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Variation {
    Linear,         // 0
    Handkerchief,   // 6
    Julia,          // 13
    Bent,           // 14
    Popcorn,        // 17
    Pdj(PDJParams), // 24
}

pub fn calculate_variation_transform(
    p: Vector2<f64>,
    variation: Variation,
    affine_coefs: &AffineCoefs,
    rng: &mut ChaCha12Rng,
) -> Vector2<f64> {
    match variation {
        Variation::Linear => p,
        Variation::Handkerchief => handkerchief(p),
        Variation::Julia => julia(p, rng),
        Variation::Bent => bent(p),
        Variation::Popcorn => popcorn(p, affine_coefs),
        Variation::Pdj(pdj_params) => pdj(p, pdj_params),
        _ => unimplemented!("Unknown variation"),
    }
}

pub fn calculate_variations_blend(
    p: Vector2<f64>,
    variations_and_weights: &[VariationAndWeight],
    affine_coefs: &AffineCoefs,
    rng: &mut ChaCha12Rng,
) -> Vector2<f64> {
    let mut new_p: Vector2<f64> = Vector2::zeros();
    for variation_and_weight in variations_and_weights {
        new_p += variation_and_weight.weight
            * calculate_variation_transform(p, variation_and_weight.variation, affine_coefs, rng);
    }
    new_p
}
