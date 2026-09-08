use crate::compute::compute_params::ComputeParams;
use crate::fractal_info::FractalInfo;
use crate::fractal_info::transform::affine::calculate_affine_transform;
use crate::fractal_info::transform::variation::calculate_variations_blend;
use crate::histogram::Histogram;
use nalgebra::Vector2;
use rand::distr::Uniform;
use rand::prelude::*;
use rand::rngs::ChaCha12Rng;
use rayon::prelude::*;
use rayon::{ThreadPool, ThreadPoolBuilder};
use std::debug_assert_matches;
use std::sync::{Arc, Mutex, mpsc};

pub struct ComputeState {
    thread_pool: ThreadPool,
    main_histogram: Option<Histogram>,
    sequences_compute_states: Vec<SequenceComputeState>,
    sequences_compute_state_channel: (
        mpsc::Sender<SequenceComputeState>,
        mpsc::Receiver<SequenceComputeState>,
    ),
    active_sequences_count: usize,
    merging_result_channel: (
        mpsc::Sender<(Histogram, Vec<SequenceComputeState>)>,
        mpsc::Receiver<(Histogram, Vec<SequenceComputeState>)>,
    ),
    state: State,
}

#[derive(Debug)]
enum State {
    NoWork,
    ComputeSequences,
    MergingHistograms,
}

impl ComputeState {
    pub fn new(compute_params: &ComputeParams) -> Self {
        let thread_pool = ThreadPoolBuilder::new()
            .num_threads(compute_params.threads_number)
            .build()
            .expect("Failed to build thread pool");

        let main_histogram = Some(Histogram::new(
            compute_params.histogram_resolution.x,
            compute_params.histogram_resolution.y,
        ));

        let mut sequences_compute_states = Vec::with_capacity(compute_params.sequences_number);
        let mut local_solvers_seeds_gen = ChaCha12Rng::seed_from_u64(compute_params.rng_seed);
        for _ in 0..compute_params.sequences_number {
            let rng = ChaCha12Rng::from_seed(local_solvers_seeds_gen.random());
            let psi_rng = ChaCha12Rng::from_seed(local_solvers_seeds_gen.random());
            sequences_compute_states.push(SequenceComputeState::new(
                compute_params.fractal_info.clone(),
                compute_params.histogram_resolution,
                compute_params.burn_iterations_count,
                rng,
                psi_rng,
            ));
        }
        let sequences_compute_state_channel = mpsc::channel();
        let merging_result_channel = mpsc::channel();

        Self {
            thread_pool,
            main_histogram,
            sequences_compute_states,
            sequences_compute_state_channel,
            active_sequences_count: 0,
            merging_result_channel,
            state: State::NoWork,
        }
    }

    pub fn run_iterations_in_current_sequences(&mut self, iterations_number: u64) {
        debug_assert_matches!(self.state, State::NoWork);
        debug_assert!(iterations_number > 0);
        self.state = State::ComputeSequences;
        let iterations_per_sequence_base =
            iterations_number / self.sequences_compute_states.len() as u64;
        let iterations_per_thread_rem =
            iterations_number % self.sequences_compute_states.len() as u64;

        let sequences_compute_states = std::mem::take(&mut self.sequences_compute_states);
        self.active_sequences_count = sequences_compute_states.len();
        for (i, mut sequences_compute_state) in sequences_compute_states.into_iter().enumerate() {
            let mut iterations_number = iterations_per_sequence_base;
            if i == 0 {
                iterations_number += iterations_per_thread_rem;
            };

            let sender = self.sequences_compute_state_channel.0.clone();
            self.thread_pool.spawn(move || {
                sequences_compute_state.compute(iterations_number);
                sender
                    .send(sequences_compute_state)
                    .expect("Failed to send compute state from thread");
            });
        }
    }

    /// Return true if merging occurs
    pub fn try_poll_and_merge(&mut self) -> bool {
        match self.state {
            State::ComputeSequences => {
                // Pool
                for sequences_compute_state in self.sequences_compute_state_channel.1.try_iter() {
                    self.sequences_compute_states.push(sequences_compute_state);
                    self.active_sequences_count -= 1;
                }

                // Start merging
                if self.active_sequences_count == 0 {
                    // Sequences computed, merge
                    self.state = State::MergingHistograms;
                    let mut main_histogram = self.main_histogram.take().unwrap();
                    let sequences_compute_states =
                        std::mem::take(&mut self.sequences_compute_states);
                    let sender = self.merging_result_channel.0.clone();
                    self.thread_pool.spawn(move || {
                        for y in 0..main_histogram.height() {
                            for x in 0..main_histogram.width() {
                                let mut counts_sum: u64 = 0;
                                let mut colors_sum_with_count: f64 = 0.0;
                                for sequence_compute_state in &sequences_compute_states {
                                    let sequence_cell = sequence_compute_state.histogram.get(x, y);
                                    counts_sum += sequence_cell.count;
                                    colors_sum_with_count +=
                                        sequence_cell.color * sequence_cell.count as f64;
                                }
                                let mut main_cell = main_histogram.get_mut(x, y);
                                main_cell.count = counts_sum;
                                main_cell.color = colors_sum_with_count / counts_sum as f64;
                            }
                        }

                        sender.send((main_histogram, sequences_compute_states));
                    });
                }
                false
            }
            State::MergingHistograms => {
                // Check merging result and return sequence compute states
                if let Ok((main_histogram, sequences_compute_states)) =
                    self.merging_result_channel.1.try_recv()
                {
                    self.state = State::NoWork;
                    self.main_histogram = Some(main_histogram);
                    self.sequences_compute_states = sequences_compute_states;
                    return true;
                }
                false
            }
            State::NoWork => false,
        }
    }

    pub fn get_histogram(&self) -> Option<&Histogram> {
        self.main_histogram.as_ref()
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
    psi_rng_uniform: Uniform<f64>,
}

impl SequenceComputeState {
    pub fn new(
        fractal_info: Arc<FractalInfo>,
        histogram_resolution: Vector2<usize>,
        burn_iterations_count: u64,
        mut rng: ChaCha12Rng,
        psi_rng: ChaCha12Rng,
    ) -> Self {
        let p: Vector2<f64> =
            Vector2::new(rng.random_range(-1.0..=1.0), rng.random_range(-1.0..=1.0));
        let histogram = Histogram::new(histogram_resolution.x, histogram_resolution.y);
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
            psi_rng_uniform: Uniform::new_inclusive(0.0, 1.0).unwrap(),
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
            self.p = calculate_variations_blend(
                self.p,
                &transform.variations_and_weights,
                &transform.affine_coefs,
                &mut self.rng,
            );

            // Perform post transform

            if !self
                .compute_area
                .contains(euclid::Point2D::new(self.p.x, self.p.y))
            {
                // Point not in fractal area, skip
                // Don't put in histogram
                self.iterations_count += 1;
                solved_iterations_count += 1;
                continue;
            }
            if self.iterations_count < self.burn_iterations_count {
                // Don't put in histogram, skip
                self.iterations_count += 1;
                solved_iterations_count += 1;
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
            debug_assert!(self.color <= 1.0);
            histogram_cell.color = (histogram_cell.color + self.color) / 2.0;
            debug_assert!(histogram_cell.color <= 1.0);

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
    let histogram_coord_y = (histogram_coord_y_f.floor() as usize).clamp(0, histogram_size.y - 1);

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
