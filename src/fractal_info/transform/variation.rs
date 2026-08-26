use nalgebra as na;
use nalgebra::Vector2;

#[derive(Clone)]
pub struct VariationAndWeight {
    pub variation: Variation,
    pub weight: f64,
}

#[derive(Copy, Clone)]
pub enum Variation {
    Linear,
}

pub fn calculate_variation_transform(p: Vector2<f64>, variation: Variation) -> Vector2<f64> {
    match variation {
        Variation::Linear => p,
    }
}

pub fn calculate_variations_blend(
    p: Vector2<f64>,
    variations_and_weights: &[VariationAndWeight],
) -> Vector2<f64> {
    let mut new_p: Vector2<f64> = p;
    for variation_and_weight in variations_and_weights {
        new_p += variation_and_weight.weight
            * calculate_variation_transform(p, variation_and_weight.variation);
    }
    new_p
}
