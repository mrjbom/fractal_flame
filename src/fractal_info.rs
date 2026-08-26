use crate::fractal_info::affine::AffineCoefs;
use crate::fractal_info::variation::VariationAndWeight;

mod affine;
mod variation;

struct FractalInfo {
    affine_coefs: AffineCoefs,
    variations_and_weights: Vec<VariationAndWeight>,
}
