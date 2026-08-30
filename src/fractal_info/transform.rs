pub mod affine;
pub mod variation;

use affine::AffineCoefs;
use variation::VariationAndWeight;

#[derive(Clone)]
pub struct Transform {
    pub affine_coefs: AffineCoefs,
    pub variations_and_weights: Vec<VariationAndWeight>,
    /// Offset in palette
    pub color: f64,
    /// Weight of function color in mixing with histogram cell color <br>
    /// if < 0.5 - cell color stronger <br>
    /// if > 0.5 - function color stronger
    pub color_speed: f64,
}
