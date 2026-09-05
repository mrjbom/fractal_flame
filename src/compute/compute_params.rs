use crate::fractal_info::FractalInfo;
use nalgebra::Vector2;
use std::sync::Arc;

pub struct ComputeParams {
    pub threads_number: usize,
    pub sequences_number: usize,
    pub fractal_info: Arc<FractalInfo>,
    pub histogram_resolution: Vector2<usize>,
    pub burn_iterations_count: u64,
    pub rng_seed: u64,
}
