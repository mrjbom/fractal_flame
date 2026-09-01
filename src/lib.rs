#![allow(unused)]

use crate::state::State;
use eframe::egui::load::SizedTexture;
use eframe::egui::{
    Align, Color32, ColorImage, Context, Image, ImageSource, Layout, TextureHandle, TextureOptions,
    Ui, ViewportBuilder,
};
use eframe::{Frame, egui};
use image::RgbaImage;
use std::fs;

const DEFAULT_WINDOW_SIZE: (usize, usize) = (1280, 720);
const DEFAULT_IMAGE_SIZE: (usize, usize) = (720, 720);

mod fractal_info;
mod histogram;
mod state;

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
    state: State,
}

impl App {
    pub fn new(creation_context: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&creation_context.egui_ctx);
        let state = State::new(creation_context);
        Self { state }
    }
}

impl eframe::App for App {
    fn logic(&mut self, _ctx: &Context, _frame: &mut Frame) {}

    fn ui(&mut self, ui: &mut Ui, frame: &mut Frame) {
        self.state.draw_ui(ui, frame);
    }
}
