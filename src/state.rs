mod compute;
mod render;

use crate::fractal_info::FractalInfo;
use compute::Compute;
use eframe::egui_wgpu::RenderState;
use render::Render;

struct State {
    fractal_info: FractalInfo,
    compute: Compute,
    render: Render,
}
