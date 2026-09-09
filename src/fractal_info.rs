use crate::fractal_info::transform::affine::AffineCoefs;
use crate::fractal_info::transform::variation::Variation;
use crate::fractal_info::transform::variation::variations::PDJParams;
use nalgebra::Vector2;
use std::sync::OnceLock;
use transform::Transform;

pub mod transform;

pub static DEFAULT_FRACTAL_INFO: OnceLock<FractalInfo> = OnceLock::new();

#[derive(Clone, Debug)]
pub struct TransformAndProbability {
    pub transform: Transform,
    pub probability: f32,
}

#[derive(Clone, Debug)]
pub struct FractalInfo {
    // XML "The width and height in pixels of the output image."
    // Size specified in .flame file, not my rendering
    pub specified_image_size: Vector2<u32>,
    pub specified_scale: f64,
    pub specified_center: Vector2<f64>,
    pub transforms_and_probabilities: Vec<TransformAndProbability>,
}

pub fn init_default_fractal_info() {
    /*
    // Affine only test
    DEFAULT_FRACTAL_INFO
        .set(FractalInfo {
            specified_image_size: Vector2::new(600, 600),
            specified_scale: 192.0,
            specified_center: Vector2::zeros(),
            transforms_and_probabilities: vec![TransformAndProbability {
                transform: Transform {
                    affine_coefs: AffineCoefs {
                        a: 1.0,
                        b: 0.0,
                        c: 0.01,
                        d: 0.0,
                        e: 1.0,
                        f: -0.02,
                    },
                    variations_and_weights: vec![(Variation::Linear, 1.0).into()],
                    color: 1.0,
                    color_speed: 0.5,
                },
                probability: 1.0,
            }],
        })
        .expect("Failed to init default fractal info once lock");
     */
    /*
    // Julia test
    DEFAULT_FRACTAL_INFO
        .set(FractalInfo {
            specified_image_size: Vector2::new(600, 600),
            specified_scale: 192.0,
            specified_center: Vector2::zeros(),
            transforms_and_probabilities: vec![TransformAndProbability {
                transform: Transform {
                    affine_coefs: AffineCoefs {
                        a: 1.0,
                        b: 0.0,
                        c: 0.0,
                        d: 0.0,
                        e: 1.0,
                        f: 0.0,
                    },
                    variations_and_weights: vec![(Variation::Linear, 1.0).into(), (Variation::Julia, 1.0).into()],
                    color: 1.0,
                    color_speed: 0.5,
                },
                probability: 1.0,
            }],
        })
        .expect("Failed to init default fractal info once lock");
     */
    /*
    // PDJ test
    DEFAULT_FRACTAL_INFO
        .set(FractalInfo {
            specified_image_size: Vector2::new(600, 600),
            specified_scale: 54.0,
            specified_center: Vector2::zeros(),
            transforms_and_probabilities: vec![TransformAndProbability {
                transform: Transform {
                    affine_coefs: AffineCoefs {
                        a: 1.0,
                        b: 0.0,
                        c: 0.0,
                        d: 0.0,
                        e: 1.0,
                        f: 0.0,
                    },
                    variations_and_weights: vec![(Variation::Pdj(PDJParams {
                        a: 1.0,
                        b: 2.0,
                        c: 2.0,
                        d: 1.0,
                    }), 1.0).into()],
                    color: 1.0,
                    color_speed: 0.5,
                },
                probability: 1.0,
            }],
        })
        .expect("Failed to init default fractal info once lock");

     */
    // Sphere, not working
    DEFAULT_FRACTAL_INFO
        .set(FractalInfo {
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
                        color: 1.0,
                        color_speed: 0.5,
                    },
                    probability: 0.564534951145298,
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
                        color: 1.0,
                        color_speed: 0.5,
                    },
                    probability: 0.0131350067581356,
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
                                Variation::Pdj(PDJParams {
                                    a: 1.09358,
                                    b: 2.13048,
                                    c: 2.54127,
                                    d: 2.37267,
                                }),
                                1.0,
                            )
                                .into(),
                        ],
                        color: 1.0,
                        color_speed: 0.5,
                    },
                    probability: 0.422330042096567,
                },
            ],
        })
        .expect("Failed to init default fractal info once lock");
}
