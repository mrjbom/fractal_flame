#![allow(unused)]

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
mod fractal_solver;
mod histogram;

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
    image_texture: TextureHandle,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        fractal_info::init_default_fractal_info();
        let image_data: Vec<Color32> =
            vec![Color32::BLACK; DEFAULT_IMAGE_SIZE.0 * DEFAULT_IMAGE_SIZE.1];
        let color_image = ColorImage::new([DEFAULT_IMAGE_SIZE.0, DEFAULT_IMAGE_SIZE.1], image_data);
        let image_texture =
            cc.egui_ctx
                .load_texture("image", color_image, TextureOptions::default());
        Self { image_texture }
    }

    fn draw_top_panel(&mut self, ui: &mut Ui) {
        egui::Panel::top("top_panel").show(ui, |ui| {
            ui.label("Top panel");
        });
    }

    fn draw_bottom_panel(&mut self, ui: &mut Ui) {
        egui::Panel::bottom("bottom_panel").show(ui, |ui| {
            ui.label("Bottom panel");
        });
    }

    fn draw_right_panel(&mut self, ui: &mut Ui) {
        egui::Panel::right("right_panel")
            .resizable(false)
            .show(ui, |ui| {
                ui.label("Right panel");
            });
    }

    fn draw_central_panel(&mut self, ui: &mut Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add(Image::from_texture(&self.image_texture).shrink_to_fit())
            });
        });
    }
}

impl eframe::App for App {
    fn logic(&mut self, _ctx: &Context, _frame: &mut Frame) {}

    fn ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
        if ui.ctx().cumulative_frame_nr() == 0 {
            ui.ctx().request_discard("Start");
        }
        self.draw_top_panel(ui);
        self.draw_bottom_panel(ui);
        self.draw_right_panel(ui);
        self.draw_central_panel(ui);
    }
}
