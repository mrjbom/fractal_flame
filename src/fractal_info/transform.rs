pub mod affine;
pub mod variation;

use affine::AffineCoefs;
use variation::VariationAndWeight;

#[derive(Clone, Debug)]
pub struct Transform {
    pub affine_coefs: AffineCoefs,
    pub variations_and_weights: Vec<VariationAndWeight>,
    /// Offset in palette
    pub color: f64,
    /// Weight of transform color in mixing with previous point color<br>
    /// if > 0.5 - transform color stronger
    /// if < 0.5 - previous point color stronger <br>
    pub color_speed: f64,
}
