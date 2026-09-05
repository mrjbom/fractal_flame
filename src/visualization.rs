use crate::visualization::visualization_params::VisualizationParams;

pub mod visualization_params;

pub struct Visualization {
    pub visualization_params: VisualizationParams,
}

impl Visualization {
    pub fn new(visualization_params: VisualizationParams) -> Self {
        Self {
            visualization_params,
        }
    }
}
