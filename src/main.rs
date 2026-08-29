use eframe::egui::{Context, Ui, ViewportBuilder};
use eframe::{Frame, egui};

const DEFAULT_WINDOW_SIZE: (usize, usize) = (1280, 720);

fn main() -> eframe::Result {
    env_logger::init();
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

struct App {}

impl App {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {}
    }
}

impl eframe::App for App {
    fn logic(&mut self, _ctx: &Context, _frame: &mut Frame) {}

    fn ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
        egui::Panel::top("top_panel").show(ui, |ui| {
            ui.label("Top panel test");
        });
        egui::Panel::right("right_panel")
            .resizable(false)
            .show(ui, |ui| {
                ui.label("Right panel test");
            });
        egui::Panel::bottom("bottom_panel").show(ui, |ui| {
            ui.label("Bottom panel test");
        });
        egui::CentralPanel::default().show(ui, |ui| {
            ui.label("Central panel test");
        });
    }
}
