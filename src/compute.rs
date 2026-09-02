use crate::DEFAULT_IMAGE_SIZE;
use crate::compute::compute_params::ComputeParams;
use crate::compute::compute_state::ComputeState;
use crate::fractal_info::FractalInfo;
use rand::RngExt;
use rand::rngs::ChaCha12Rng;
use std::sync::Arc;

mod compute_params;
mod compute_state;

pub struct Compute {
    compute_params: ComputeParams,
    compute_state: ComputeState,
}

impl Compute {
    pub fn new(fractal_info: Arc<FractalInfo>, main_rng: &mut ChaCha12Rng) -> Self {
        let compute_params = ComputeParams {
            threads_number: std::thread::available_parallelism()
                .expect("Failed to get available parallelism data")
                .get(),
            sequences_number: 1,
            fractal_info: Arc::clone(&fractal_info),
            histogram_width: DEFAULT_IMAGE_SIZE.0,
            histogram_height: DEFAULT_IMAGE_SIZE.1,
            burn_iterations_count: 15,
            rng_seed: main_rng.random(),
        };
        let compute_state = ComputeState::new(&compute_params);

        Self {
            compute_params,
            compute_state,
        }
    }
}
