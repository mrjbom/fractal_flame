use nalgebra as na;
use nalgebra::Vector2;

#[derive(Clone)]
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

#[derive(Copy, Clone)]
pub enum Variation {
    Linear,                                 // 0
    Julia,                                  // 13
    Popcorn,                                // 17
    Pdj { a: f64, b: f64, c: f64, d: f64 }, // 24
}

pub fn calculate_variation_transform(p: Vector2<f64>, variation: Variation) -> Vector2<f64> {
    match variation {
        Variation::Linear => p,
        _ => unimplemented!("Unknown variation"),
    }
}

pub fn calculate_variations_blend(
    p: Vector2<f64>,
    variations_and_weights: &[VariationAndWeight],
) -> Vector2<f64> {
    let mut new_p: Vector2<f64> = Vector2::zeros();
    for variation_and_weight in variations_and_weights {
        new_p += variation_and_weight.weight
            * calculate_variation_transform(p, variation_and_weight.variation);
    }
    new_p
}
