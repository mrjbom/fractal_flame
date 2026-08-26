use transform::Transform;

pub mod transform;

#[derive(Clone)]
pub struct TransformAndProbability {
    pub transform: Transform,
    pub probability: f32,
}

#[derive(Clone)]
pub struct FractalInfo {
    pub transforms_and_probabilities: Vec<TransformAndProbability>,
}
