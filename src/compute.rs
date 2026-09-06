use crate::DEFAULT_IMAGE_SIZE;
use crate::compute::compute_params::ComputeParams;
use crate::compute::compute_state::ComputeState;
use crate::fractal_info::FractalInfo;
use rand::RngExt;
use rand::rngs::ChaCha12Rng;
use std::sync::Arc;

pub mod compute_params;
mod compute_state;

pub struct Compute {
    compute_params: ComputeParams,
    compute_state: ComputeState,
}

impl Compute {
    pub fn new(compute_params: ComputeParams, main_rng: &mut ChaCha12Rng) -> Self {
        let compute_state = ComputeState::new(&compute_params);

        Self {
            compute_params,
            compute_state,
        }
    }

    pub fn run_compute_iterations(&mut self, iterations_number: u64) {
        self.compute_state
            .run_iterations_in_current_sequences(iterations_number);
    }
}
