use crate::fractal_info::transform::affine::AffineCoefs;
use crate::fractal_info::transform::variation::{Variation, VariationAndWeight};
use nalgebra::Vector2;
use std::sync::OnceLock;
use transform::Transform;

pub mod transform;

pub static DEFAULT_FRACTAL_INFO: OnceLock<FractalInfo> = OnceLock::new();

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

pub fn init_default_fractal_info() {
    DEFAULT_FRACTAL_INFO.set(FractalInfo {
        specified_image_size: Vector2::new(600, 600),
        specified_scale: 144.0,
        specified_center: Vector2::zeros(),
        transforms_and_probabilities: vec![
            TransformAndProbability {
                transform: Transform {
                    affine_coefs: AffineCoefs {
                        a: -1.381068,
                        b: -1.381068,
                        c: 0.0,
                        d: 1.381068,
                        e: -1.381068,
                        f: 0.0,
                    },
                    variations_and_weights: vec![(Variation::Julia, 1.0).into()],
                    color: 0.13,
                    color_speed: 0.5,
                },
                probability: 0.56453495,
            },
            TransformAndProbability {
                transform: Transform {
                    affine_coefs: AffineCoefs {
                        a: 0.031393,
                        b: 0.031367,
                        c: 0.0,
                        d: -0.031367,
                        e: 0.031393,
                        f: 0.0,
                    },
                    variations_and_weights: vec![
                        (Variation::Linear, 1.0).into(),
                        (Variation::Popcorn, 1.0).into(),
                    ],
                    color: 0.844,
                    color_speed: 0.5,
                },
                probability: 0.013135,
            },
            TransformAndProbability {
                transform: Transform {
                    affine_coefs: AffineCoefs {
                        a: 1.51523,
                        b: -3.048677,
                        c: 0.724135,
                        d: 0.740356,
                        e: -1.455964,
                        f: -0.362059,
                    },
                    variations_and_weights: vec![
                        (
                            Variation::Pdj {
                                a: 1.09358,
                                b: 2.13048,
                                c: 2.54127,
                                d: 2.37267,
                            },
                            1.0,
                        )
                            .into(),
                    ],
                    color: 0.0,
                    color_speed: 0.5,
                },
                probability: 0.42233,
            },
        ],
    });
}
