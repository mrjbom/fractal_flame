mod compute;
mod render;

use crate::fractal_info::{DEFAULT_FRACTAL_INFO, FractalInfo};
use crate::state::compute::compute_state::ComputeStateInitInfo;
use crate::{DEFAULT_IMAGE_SIZE, fractal_info};
use compute::Compute;
use eframe::Frame;
use eframe::egui::Ui;
use eframe::egui_wgpu::RenderState;
use rand::rngs::ChaCha12Rng;
use rand::{RngExt, SeedableRng};
use render::Render;
use std::sync::Arc;

pub struct State {
    main_rng: ChaCha12Rng,
    fractal_info: Arc<FractalInfo>,
    compute: Compute,
    render: Render,
}

impl State {
    pub fn new(creation_context: &eframe::CreationContext) -> Self {
        let mut main_rng = ChaCha12Rng::from_seed(rand::random());
        // Load fractal info
        fractal_info::init_default_fractal_info();
        let fractal_info = Arc::new(DEFAULT_FRACTAL_INFO.get().cloned().unwrap());

        // Init compute state
        let compute_state_init_info = ComputeStateInitInfo {
            sequences_number: std::thread::available_parallelism()
                .expect("Failed to get available parallelism data")
                .get(),
            fractal_info: Arc::clone(&fractal_info),
            histogram_width: DEFAULT_IMAGE_SIZE.0,
            histogram_height: DEFAULT_IMAGE_SIZE.1,
            histogram_initial_color: 0.0,
            burn_iterations_count: 15,
            rng_seed: main_rng.random(),
        };
        let compute = Compute::new(compute_state_init_info);

        // Init render and ui state
        let render = Render::new(creation_context);

        Self {
            main_rng,
            fractal_info,
            compute,
            render,
        }
    }

    pub fn draw_ui(&mut self, ui: &mut Ui, frame: &mut Frame) {
        self.render.ui_state.draw_ui(ui, frame);
    }
}
