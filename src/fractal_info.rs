use nalgebra::Vector2;
use transform::Transform;

pub mod transform;

#[derive(Clone)]
pub struct TransformAndProbability {
    pub transform: Transform,
    pub probability: f32,
}

#[derive(Clone)]
pub struct FractalInfo {
    // XML "The width and height in pixels of the output image."
    // Size specified in .flame file, not my rendering
    pub specified_image_size: Vector2<u32>,
    pub specified_scale: f64,
    pub specified_center: Vector2<f64>,
    pub transforms_and_probabilities: Vec<TransformAndProbability>,
}
