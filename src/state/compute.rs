use crate::state::compute::compute_state::ComputeStateInitInfo;
use compute_state::ComputeState;

pub mod compute_state;

pub struct Compute {
    compute_state: ComputeState,
}

impl Compute {
    pub fn new(compute_state_init_info: ComputeStateInitInfo) -> Self {
        let compute_state = ComputeState::new(compute_state_init_info);
        Self { compute_state }
    }
}
