#![allow(unused)]

use crate::compute::Compute;
use crate::compute::compute_params::ComputeParams;
use crate::fractal_info::{DEFAULT_FRACTAL_INFO, FractalInfo};
use crate::ui_state::UiState;
use crate::visualization::Visualization;
use crate::visualization::visualization_params::VisualizationParams;
use eframe::Frame;
use eframe::egui::{Color32, ColorImage, Context, TextureOptions, Ui, ViewportBuilder};
use nalgebra::Vector2;
use rand::rngs::ChaCha12Rng;
use rand::{RngExt, SeedableRng};
use std::sync::Arc;

const DEFAULT_WINDOW_SIZE: Vector2<usize> = Vector2::new(1280, 720);
const DEFAULT_IMAGE_SIZE: Vector2<usize> = Vector2::new(720, 720);

mod compute;
mod fractal_info;
mod histogram;
mod ui_state;
mod visualization;

pub fn run() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_title("Fractal Flame")
            .with_inner_size((DEFAULT_WINDOW_SIZE.x as f32, DEFAULT_WINDOW_SIZE.y as f32)),
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
    visualization: Visualization,
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

        let visualization_params = VisualizationParams {
            image_resolution: DEFAULT_IMAGE_SIZE,
            image_quality: 1,
        };

        let compute_params = ComputeParams {
            threads_number: std::thread::available_parallelism()
                .expect("Failed to get available parallelism data")
                .get(),
            sequences_number: 30000,
            fractal_info: Arc::clone(&fractal_info),
            histogram_resolution: visualization_params.image_resolution,
            burn_iterations_count: 15,
            rng_seed: main_rng.random(),
        };

        let visualization = Visualization::new(visualization_params);
        let mut compute = Compute::new(compute_params, &mut main_rng);
        let ui_state = UiState::new(creation_context, &visualization.visualization_params);

        // Startup
        // Start default fractal calculating
        compute.run_compute_iterations(36_000_000);

        Self {
            visualization,
            compute,
            main_rng,
            fractal_info,
            ui_state,
        }
    }
}

impl eframe::App for App {
    fn logic(&mut self, _ctx: &Context, _frame: &mut Frame) {
        if self.compute.update_histogram() {
            // Test render
            let histogram = self
                .compute
                .get_histogram()
                .expect("Failed to get histogram");
            let mut image_data: Vec<Color32> =
                Vec::with_capacity(histogram.width() * histogram.height());
            for y in 0..histogram.height() {
                for x in 0..histogram.width() {
                    let color = histogram.get(x, y).color;
                    if color > 0.0001 {
                        image_data.push(Color32::from_gray(255));
                    } else {
                        image_data.push(Color32::from_gray(0));
                    }
                }
            }
            let color_image = ColorImage::new(
                [
                    self.visualization.visualization_params.image_resolution.x,
                    self.visualization.visualization_params.image_resolution.y,
                ],
                image_data,
            );
            self.ui_state
                .image_texture_handle
                .set(color_image, TextureOptions::NEAREST);
        }
    }

    fn ui(&mut self, ui: &mut Ui, frame: &mut Frame) {
        self.ui_state.draw_ui(ui, frame);
        ui.ctx().request_repaint();
    }
}
