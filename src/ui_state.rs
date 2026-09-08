use crate::DEFAULT_IMAGE_SIZE;
use crate::visualization::visualization_params::VisualizationParams;
use eframe::egui::{Color32, ColorImage, Image, TextureHandle, TextureOptions, Ui};
use eframe::{Frame, egui};

pub struct UiState {
    pub image_texture_handle: TextureHandle,
}

impl UiState {
    pub fn new(
        creation_context: &eframe::CreationContext,
        visualization_params: &VisualizationParams,
    ) -> Self {
        let image_data: Vec<Color32> = vec![
            Color32::BLACK;
            visualization_params.image_resolution.x
                * visualization_params.image_resolution.y
        ];
        let color_image = ColorImage::new(
            [
                visualization_params.image_resolution.x,
                visualization_params.image_resolution.y,
            ],
            image_data,
        );
        let image_texture_handle =
            creation_context
                .egui_ctx
                .load_texture("image", color_image, TextureOptions::NEAREST);
        Self {
            image_texture_handle,
        }
    }

    pub fn draw_ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
        if ui.ctx().cumulative_frame_nr() == 0 {
            ui.ctx().request_discard("Startup");
        }
        self.draw_top_panel(ui);
        self.draw_bottom_panel(ui);
        self.draw_right_panel(ui);
        self.draw_central_panel(ui);
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
                ui.add(Image::from_texture(&self.image_texture_handle).shrink_to_fit())
            });
        });
    }
}
