#![allow(unused)]

use crate::compute::Compute;
use crate::fractal_info::{DEFAULT_FRACTAL_INFO, FractalInfo};
use crate::render::Render;
use crate::ui_state::UiState;
use eframe::Frame;
use eframe::egui::{Context, Ui, ViewportBuilder};
use rand::SeedableRng;
use rand::rngs::ChaCha12Rng;
use std::sync::Arc;

const DEFAULT_WINDOW_SIZE: (usize, usize) = (1280, 720);
const DEFAULT_IMAGE_SIZE: (usize, usize) = (720, 720);

mod compute;
mod fractal_info;
mod histogram;
mod render;
mod ui_state;

pub fn run() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_title("Fractal Flame")
            .with_inner_size((DEFAULT_WINDOW_SIZE.0 as f32, DEFAULT_WINDOW_SIZE.1 as f32)),
        centered: true,
        ..Default::default()
    };
    eframe::run_native(
        "FractalFlameApp",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}

pub struct App {
    render: Render,
    compute: Compute,
    main_rng: ChaCha12Rng,
    fractal_info: Arc<FractalInfo>,
    ui_state: UiState,
}

impl App {
    pub fn new(creation_context: &eframe::CreationContext) -> Self {
        egui_extras::install_image_loaders(&creation_context.egui_ctx);
        let mut main_rng = ChaCha12Rng::from_seed(rand::random());
        // Load fractal info
        fractal_info::init_default_fractal_info();
        let fractal_info = Arc::new(DEFAULT_FRACTAL_INFO.get().cloned().unwrap());

        let render = Render::new();

        let ui_state = UiState::new(creation_context);
        let compute = Compute::new(Arc::clone(&fractal_info), &mut main_rng);

        Self {
            render,
            compute,
            main_rng,
            fractal_info,
            ui_state,
        }
    }

    pub fn startup(&mut self) {
        // Set default params
    }
}

impl eframe::App for App {
    fn logic(&mut self, _ctx: &Context, _frame: &mut Frame) {}

    fn ui(&mut self, ui: &mut Ui, frame: &mut Frame) {
        self.ui_state.draw_ui(ui, frame);
    }
}
