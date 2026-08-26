mod affine;
mod variation;

use affine::AffineCoefs;
use variation::VariationAndWeight;

#[derive(Clone)]
pub struct Transform {
    pub affine_coefs: AffineCoefs,
    pub variations_and_weights: Vec<VariationAndWeight>,
    // offset in palette
    pub color: f64,
}
