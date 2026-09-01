use crate::state::render::ui_state::UiState;
use render_state::RenderState;

mod render_state;
mod ui_state;

pub struct Render {
    pub render_state: RenderState,
    pub ui_state: UiState,
}

impl Render {
    pub fn new(creation_context: &eframe::CreationContext) -> Self {
        let render_state = RenderState::new();
        let ui_state = UiState::new(creation_context);
        Self {
            render_state,
            ui_state,
        }
    }
}
