use crate::fractal_info::FractalInfo;
use crate::histogram::Histogram;
use nalgebra::Vector2;
use rand::prelude::*;
use rand::rngs::ChaCha12Rng;
use rayon::prelude::*;

// Monte Carlo parallel solver
pub struct FractalSolver {
    fractal_solver_init_info: FractalSolverInitInfo,
    local_fractal_solvers: Vec<LocalFractalSolver>,
}

impl FractalSolver {
    pub fn new(fractal_solver_init_info: FractalSolverInitInfo) -> Self {
        let mut local_fractal_solvers =
            Vec::with_capacity(fractal_solver_init_info.sequences_number);
        let mut local_solvers_seeds_gen =
            ChaCha12Rng::seed_from_u64(fractal_solver_init_info.rng_seed);
        for _ in 0..local_fractal_solvers.len() {
            let rng = ChaCha12Rng::from_seed(local_solvers_seeds_gen.random());
            let psi_rng = ChaCha12Rng::from_seed(local_solvers_seeds_gen.random());
            local_fractal_solvers.push(LocalFractalSolver::new(
                fractal_solver_init_info.fractal_info.clone(),
                fractal_solver_init_info.histogram_width,
                fractal_solver_init_info.histogram_height,
                fractal_solver_init_info.histogram_initial_color,
                fractal_solver_init_info.burn_iterations_count,
                rng,
                psi_rng,
            ));
        }
        Self {
            fractal_solver_init_info,
            local_fractal_solvers,
        }
    }
}

struct LocalFractalSolver {
    // x, y
    p: Vector2<f64>,
    color: f64,
    iterations_count: u64,
    burn_iterations_count: u64,
    histogram: Histogram,
    fractal_info: FractalInfo,
    rng: ChaCha12Rng,
    psi_rng: ChaCha12Rng,
}

impl LocalFractalSolver {
    pub fn new(
        fractal_info: FractalInfo,
        histogram_width: usize,
        histogram_height: usize,
        histogram_initial_color: f64,
        burn_iterations_count: u64,
        mut rng: ChaCha12Rng,
        psi_rng: ChaCha12Rng,
    ) -> Self {
        let p: Vector2<f64> =
            Vector2::new(rng.random_range(-1.0..=1.0), rng.random_range(-1.0..=1.0));
        let histogram = Histogram::new(histogram_width, histogram_height, histogram_initial_color);
        LocalFractalSolver {
            p,
            color: 0.0,
            iterations_count: 0,
            burn_iterations_count: 0,
            histogram,
            fractal_info,
            rng,
            psi_rng,
        }
    }

    pub fn solve(&mut self, iterations_number: u64) {
        while self.iterations_count < iterations_number {
            // Select random transform

            if self.iterations_count == 0 {
                // Color of func
                //self.color
            }
            if self.iterations_count < self.burn_iterations_count {
                // Don't put data in the histogram
            }
            self.iterations_count += 1;
        }
    }
}

pub struct FractalSolverInitInfo {
    pub sequences_number: usize,
    pub iterations_number: usize,
    pub fractal_info: FractalInfo,
    pub histogram_width: usize,
    pub histogram_height: usize,
    pub histogram_initial_color: f64,
    pub burn_iterations_count: u64,
    pub rng_seed: u64,
}
