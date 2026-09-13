use crate::render::render_params::RenderParams;

pub mod render_params;

pub struct Render {
    pub render_params: RenderParams,
}

impl Render {
    pub fn new(render_params: RenderParams) -> Self {
        Self { render_params }
    }
}
