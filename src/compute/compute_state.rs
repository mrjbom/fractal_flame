use crate::compute::compute_params::ComputeParams;
use crate::fractal_info::FractalInfo;
use crate::fractal_info::transform::affine::calculate_affine_transform;
use crate::fractal_info::transform::variation::calculate_variations_blend;
use crate::histogram::Histogram;
use nalgebra::Vector2;
use rand::prelude::*;
use rand::rngs::ChaCha12Rng;
use rayon::prelude::*;
use rayon::{ThreadPool, ThreadPoolBuilder};
use std::ops::AddAssign;
use std::sync::{Arc, mpsc};

pub struct ComputeState {
    thread_pool: ThreadPool,
    main_histogram: Histogram,
    sequences_compute_states: Vec<SequenceComputeState>,
    sequences_compute_state_channel: (
        mpsc::Sender<SequenceComputeState>,
        mpsc::Receiver<SequenceComputeState>,
    ),
    sequences_compute_in_progress: bool,
}

impl ComputeState {
    pub fn new(compute_params: &ComputeParams) -> Self {
        let thread_pool = ThreadPoolBuilder::new()
            .num_threads(compute_params.threads_number)
            .build()
            .expect("Failed to build thread pool");

        let main_histogram = Histogram::new(
            compute_params.histogram_width,
            compute_params.histogram_height,
        );

        let mut sequences_compute_states = Vec::with_capacity(compute_params.threads_number);
        let mut local_solvers_seeds_gen = ChaCha12Rng::seed_from_u64(compute_params.rng_seed);
        for _ in 0..sequences_compute_states.len() {
            let rng = ChaCha12Rng::from_seed(local_solvers_seeds_gen.random());
            let psi_rng = ChaCha12Rng::from_seed(local_solvers_seeds_gen.random());
            sequences_compute_states.push(SequenceComputeState::new(
                compute_params.fractal_info.clone(),
                compute_params.histogram_width,
                compute_params.histogram_height,
                compute_params.burn_iterations_count,
                rng,
                psi_rng,
            ));
        }
        let sequences_compute_state_channel = mpsc::channel();

        Self {
            thread_pool,
            main_histogram,
            sequences_compute_states,
            sequences_compute_state_channel,
            sequences_compute_in_progress: false,
        }
    }

    pub fn run_iterations_in_current_sequences(&mut self, iterations_number: u64) {
        debug_assert!(!self.sequences_compute_in_progress);
        // Do iterations_number iterations in compute states using thread_pool
        for i in 0..self.sequences_compute_states.len() {
            let mut sequence_compute_state = self.sequences_compute_states.remove(i);
            let sender = self.sequences_compute_state_channel.0.clone();
            self.thread_pool.spawn(move || {
                sequence_compute_state.compute(iterations_number);
                sender.send(sequence_compute_state).unwrap();
            });
        }
        self.sequences_compute_in_progress = true;
    }

    pub fn try_receive_sequences(&mut self, compute_params: &ComputeParams) -> bool {
        if !self.sequences_compute_in_progress {
            return true;
        }

        for sequence_compute_state in self.sequences_compute_state_channel.1.try_iter() {
            self.sequences_compute_states.push(sequence_compute_state);
        }

        if self.sequences_compute_states.len() == compute_params.sequences_number {
            self.sequences_compute_in_progress = false;
        }
        !self.sequences_compute_in_progress
    }

    pub fn merge_local_sequences_histograms_to_main(&mut self) {
        debug_assert!(!self.sequences_compute_in_progress);
        self.main_histogram.clear();
        for y in 0..self.main_histogram.height() {
            for x in 0..self.main_histogram.width() {
                let main_cell = self.main_histogram.get_mut(x, y);
                let mut color_sum = 0.0;
                for sequence_compute_state in &self.sequences_compute_states {
                    let sequence_compute_cell = sequence_compute_state.histogram.get(x, y);
                    main_cell.count += sequence_compute_cell.count;
                    color_sum += sequence_compute_cell.color;
                }
                main_cell.color = color_sum / self.sequences_compute_states.len() as f64;
            }
        }
    }
}

struct SequenceComputeState {
    p: Vector2<f64>,
    color: f64,
    iterations_count: u64,
    burn_iterations_count: u64,
    histogram: Histogram,
    fractal_info: Arc<FractalInfo>,
    compute_area: euclid::Box2D<f64, ()>,
    rng: ChaCha12Rng,
    psi_rng: ChaCha12Rng,
}

impl SequenceComputeState {
    pub fn new(
        fractal_info: Arc<FractalInfo>,
        histogram_width: usize,
        histogram_height: usize,
        burn_iterations_count: u64,
        mut rng: ChaCha12Rng,
        psi_rng: ChaCha12Rng,
    ) -> Self {
        let p: Vector2<f64> =
            Vector2::new(rng.random_range(-1.0..=1.0), rng.random_range(-1.0..=1.0));
        let histogram = Histogram::new(histogram_width, histogram_height);
        let compute_area_size: Vector2<f64> =
            fractal_info.specified_image_size.cast::<f64>() / fractal_info.specified_scale;
        let compute_area: euclid::Box2D<f64, ()> = euclid::Box2D::new(
            euclid::Point2D::new(
                fractal_info.specified_center.x - compute_area_size.x / 2.0,
                fractal_info.specified_center.y - compute_area_size.y / 2.0,
            ),
            euclid::Point2D::new(
                fractal_info.specified_center.x + compute_area_size.x / 2.0,
                fractal_info.specified_center.y + compute_area_size.y / 2.0,
            ),
        );

        Self {
            p,
            color: 0.0,
            iterations_count: 0,
            burn_iterations_count,
            histogram,
            fractal_info,
            compute_area,
            rng,
            psi_rng,
        }
    }

    pub fn compute(&mut self, iterations_number: u64) {
        let mut solved_iterations_count = 0;
        while solved_iterations_count < iterations_number {
            // Select random transform
            let transform = &self
                .fractal_info
                .transforms_and_probabilities
                .choose_weighted(&mut self.rng, |transform_and_probability| {
                    transform_and_probability.probability
                })
                .expect("Failed to get random transform")
                .transform;

            // Perform affine transform
            self.p = calculate_affine_transform(self.p, &transform.affine_coefs);

            // Perform variations blending
            self.p = calculate_variations_blend(self.p, &transform.variations_and_weights);

            // Perform post transform

            if !self
                .compute_area
                .contains(euclid::Point2D::new(self.p.x, self.p.y))
            {
                // Point not in fractal area, skip
                // Don't put in histogram
                self.iterations_count += 1;
                continue;
            }
            if self.iterations_count < self.burn_iterations_count {
                // Don't put in histogram, skip
                self.iterations_count += 1;
                continue;
            }

            // Calculate final transform
            // DO NOT MODIFY POINT
            // Final transform only for histogram
            let p_final = self.p;

            let histogram_coords = compute_coords_to_histogram_coords(
                p_final,
                self.compute_area,
                Vector2::new(self.histogram.width(), self.histogram.height()),
            );

            // Put point in histogram
            let histogram_cell = self
                .histogram
                .get_mut(histogram_coords.x, histogram_coords.y);
            histogram_cell.count += 1;

            if self.iterations_count == 0 {
                self.color = transform.color;
            }
            self.color = self.color * (1.0 - transform.color_speed)
                + transform.color * transform.color_speed;

            self.iterations_count += 1;
            solved_iterations_count += 1;
        }
    }
}

pub fn compute_coords_to_histogram_coords(
    p: Vector2<f64>,
    compute_area: euclid::Box2D<f64, ()>,
    histogram_size: Vector2<usize>,
) -> Vector2<usize> {
    let normalized_x = (p.x - compute_area.center().x) / compute_area.width() + 0.5;
    let normalized_y = (compute_area.center().y - p.y) / compute_area.height() + 0.5;

    let histogram_coord_x_f = normalized_x * (histogram_size.x as f64);
    let histogram_coord_y_f = normalized_y * (histogram_size.y as f64);

    let histogram_coord_x = (histogram_coord_x_f.floor() as usize).clamp(0, histogram_size.x - 1);
    let histogram_coord_y = (histogram_coord_y_f.floor() as usize).clamp(0, histogram_size.y);

    Vector2::new(histogram_coord_x, histogram_coord_y)
}

#[cfg(test)]
mod tests {
    use super::compute_coords_to_histogram_coords;
    use nalgebra::Vector2;

    #[test]
    fn test1() {
        let histogram_size: Vector2<usize> = Vector2::new(800, 600);
        let compute_area_size = Vector2::new(4.0, 3.5);
        let center = Vector2::new(1.15, -0.35);
        let compute_area: euclid::Box2D<f64, ()> = euclid::Box2D::new(
            euclid::Point2D::new(
                center.x - compute_area_size.x / 2.0,
                center.y - compute_area_size.y / 2.0,
            ),
            euclid::Point2D::new(
                center.x + compute_area_size.x / 2.0,
                center.y + compute_area_size.y / 2.0,
            ),
        );

        let p0 = Vector2::new(-0.85, 1.4);
        let r0 = compute_coords_to_histogram_coords(p0, compute_area, histogram_size);
        assert_eq!(r0, Vector2::new(0, 0));

        let p1 = Vector2::new(3.1499, 1.4);
        let r1 = compute_coords_to_histogram_coords(p1, compute_area, histogram_size);
        assert_eq!(r1, Vector2::new(799, 0));

        let p2 = Vector2::new(-0.85, -2.099);
        let r2 = compute_coords_to_histogram_coords(p2, compute_area, histogram_size);
        assert_eq!(r2, Vector2::new(0, 599));

        let p3 = Vector2::new(3.149, -2.099);
        let r3 = compute_coords_to_histogram_coords(p3, compute_area, histogram_size);
        assert_eq!(r3, Vector2::new(799, 599));
    }
}
