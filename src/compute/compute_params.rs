use crate::fractal_info::FractalInfo;
use std::sync::Arc;

pub struct ComputeParams {
    pub threads_number: usize,
    pub sequences_number: usize,
    pub fractal_info: Arc<FractalInfo>,
    pub histogram_width: usize,
    pub histogram_height: usize,
    pub burn_iterations_count: u64,
    pub rng_seed: u64,
}
