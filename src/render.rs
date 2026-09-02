use crate::render::render_params::RenderParams;

mod render_params;

pub struct Render {
    render_params: RenderParams,
}

impl Render {
    pub fn new() -> Self {
        let render_params = RenderParams {};
        Self { render_params }
    }
}
