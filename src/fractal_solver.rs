use crate::fractal_info::FractalInfo;
use crate::fractal_info::transform::affine::calculate_affine_transform;
use crate::fractal_info::transform::variation::calculate_variations_blend;
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
    p: Vector2<f64>,
    color: f64,
    iterations_count: u64,
    burn_iterations_count: u64,
    histogram: Histogram,
    fractal_info: FractalInfo,
    compute_area: euclid::Box2D<f64, ()>,
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

        LocalFractalSolver {
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

    pub fn solve(&mut self, iterations_number: u64) {
        while self.iterations_count < iterations_number {
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
    use crate::fractal_solver::compute_coords_to_histogram_coords;
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
