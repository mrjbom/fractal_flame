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
    thread_contexts: Arc<Vec<Mutex<ThreadContext>>>,
    sequences_compute_states: Vec<SequenceComputeState>,
    sequences_compute_state_channel: (
        mpsc::Sender<SequenceComputeState>,
        mpsc::Receiver<SequenceComputeState>,
    ),
    active_sequences_count: usize,
    merging_result_channel: (mpsc::Sender<Histogram>, mpsc::Receiver<Histogram>),
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

        let fractal_info = Arc::clone(&compute_params.fractal_info);

        let main_histogram = Some(Histogram::new(
            compute_params.histogram_resolution.x,
            compute_params.histogram_resolution.y,
        ));

        let mut master_seed_gen = ChaCha12Rng::seed_from_u64(compute_params.rng_seed);

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

        let thread_contexts: Vec<Mutex<ThreadContext>> = (0..compute_params.threads_number)
            .map(|_| {
                let rng = ChaCha12Rng::from_seed(master_seed_gen.random());
                let psi_rng = ChaCha12Rng::from_seed(master_seed_gen.random());
                Mutex::new(ThreadContext {
                    histogram: Histogram::new(
                        compute_params.histogram_resolution.x,
                        compute_params.histogram_resolution.y,
                    ),
                    rng,
                    psi_rng,
                    psi_rng_uniform: Uniform::new_inclusive(0.0, 1.0).unwrap(),
                    burn_iterations_count: compute_params.burn_iterations_count,
                    fractal_info: Arc::clone(&fractal_info),
                    compute_area,
                })
            })
            .collect();
        let thread_contexts = Arc::new(thread_contexts);

        let mut sequences_compute_states = Vec::with_capacity(compute_params.sequences_number);
        for _ in 0..compute_params.sequences_number {
            sequences_compute_states.push(SequenceComputeState::new(
                compute_params.fractal_info.clone(),
                compute_params.burn_iterations_count,
                &mut master_seed_gen,
            ));
        }

        let sequences_compute_state_channel = mpsc::channel();
        let merging_result_channel = mpsc::channel();

        Self {
            thread_pool,
            main_histogram,
            thread_contexts,
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
        for (i, mut sequence_compute_state) in sequences_compute_states.into_iter().enumerate() {
            let mut iterations_number = iterations_per_sequence_base;
            if i == 0 {
                iterations_number += iterations_per_thread_rem;
            };

            let sender = self.sequences_compute_state_channel.0.clone();
            let thread_contexts = Arc::clone(&self.thread_contexts);
            self.thread_pool.spawn(move || {
                let thread_index = rayon::current_thread_index()
                    .expect("Task must run inside the compute state's own thread pool");
                let mut thread_context = thread_contexts[thread_index]
                    .lock()
                    .expect("Thread context mutex poisoned");

                sequence_compute_state.compute(iterations_number, &mut thread_context);
                drop(thread_context);
                sender
                    .send(sequence_compute_state)
                    .expect("Failed to send compute state from thread");
            });
        }
    }

    /// Return true if merging occurs
    pub fn try_poll_and_merge(&mut self) -> bool {
        match self.state {
            State::ComputeSequences => {
                // Pool
                for sequence_compute_state in self.sequences_compute_state_channel.1.try_iter() {
                    self.sequences_compute_states.push(sequence_compute_state);
                    self.active_sequences_count -= 1;
                }

                // Start merging
                if self.active_sequences_count == 0 {
                    self.state = State::MergingHistograms;
                    let mut main_histogram = self.main_histogram.take().unwrap();
                    let thread_contexts = Arc::clone(&self.thread_contexts);
                    let sender = self.merging_result_channel.0.clone();
                    self.thread_pool.spawn(move || {
                        let locked_contexts: Vec<_> = thread_contexts
                            .iter()
                            .map(|ctx| ctx.lock().expect("Thread context mutex poisoned"))
                            .collect();

                        for y in 0..main_histogram.height() {
                            for x in 0..main_histogram.width() {
                                let mut counts_sum: u64 = 0;
                                let mut colors_sum_with_count: f64 = 0.0;
                                for thread_context in &locked_contexts {
                                    let cell = thread_context.histogram.get(x, y);
                                    counts_sum += cell.count;
                                    colors_sum_with_count += cell.color * cell.count as f64;
                                }
                                let mut main_cell = main_histogram.get_mut(x, y);
                                main_cell.count = counts_sum;
                                main_cell.color = colors_sum_with_count / counts_sum as f64;
                            }
                        }

                        drop(locked_contexts);
                        sender
                            .send(main_histogram)
                            .expect("Failed to send merged histogram from thread");
                    });
                }
                false
            }
            State::MergingHistograms => {
                if let Ok(main_histogram) = self.merging_result_channel.1.try_recv() {
                    self.state = State::NoWork;
                    self.main_histogram = Some(main_histogram);
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

struct ThreadContext {
    histogram: Histogram,
    rng: ChaCha12Rng,
    psi_rng: ChaCha12Rng,
    psi_rng_uniform: Uniform<f64>,
    burn_iterations_count: u64,
    fractal_info: Arc<FractalInfo>,
    compute_area: euclid::Box2D<f64, ()>,
}

struct SequenceComputeState {
    p: Vector2<f64>,
    color: f64,
    iterations_count: u64,
}

impl SequenceComputeState {
    pub fn new(
        fractal_info: Arc<FractalInfo>,
        burn_iterations_count: u64,
        seed_rng: &mut ChaCha12Rng,
    ) -> Self {
        let p: Vector2<f64> = Vector2::new(
            seed_rng.random_range(-1.0..=1.0),
            seed_rng.random_range(-1.0..=1.0),
        );

        Self {
            p,
            color: 0.0,
            iterations_count: 0,
        }
    }

    pub fn compute(&mut self, iterations_number: u64, thread_context: &mut ThreadContext) {
        let mut solved_iterations_count = 0;
        while solved_iterations_count < iterations_number {
            // Select random transform
            let transform = &thread_context
                .fractal_info
                .transforms_and_probabilities
                .choose_weighted(&mut thread_context.rng, |transform_and_probability| {
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
                &mut thread_context.rng,
            );

            // Perform post transform

            if !thread_context
                .compute_area
                .contains(euclid::Point2D::new(self.p.x, self.p.y))
            {
                self.iterations_count += 1;
                solved_iterations_count += 1;
                continue;
            }
            if self.iterations_count < thread_context.burn_iterations_count {
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
                thread_context.compute_area,
                Vector2::new(
                    thread_context.histogram.width(),
                    thread_context.histogram.height(),
                ),
            );

            let histogram_cell = thread_context
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
    let normalized_y = (p.y - compute_area.center().y) / compute_area.height() + 0.5;

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
