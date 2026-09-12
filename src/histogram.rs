use ndarray::Array2;

#[derive(Clone)]
pub struct HistogramCell {
    pub count: u64,
    pub color_sum: f64,
}

impl HistogramCell {
    pub fn new() -> Self {
        Self {
            count: 0,
            color_sum: 0.0,
        }
    }

    #[inline(always)]
    pub fn color_avg(&self) -> f64 {
        if self.count > 0 {
            self.color_sum / self.count as f64
        } else {
            0.0
        }
    }
}

#[derive(Clone)]
pub struct Histogram {
    // Use get and get_mut
    cells: Array2<HistogramCell>,
    count_max: u64,
}

impl Histogram {
    pub fn new(width: usize, height: usize) -> Self {
        // MxN, M rows, N columns
        let cells = Array2::from_shape_fn((height, width), |(_i, _j)| HistogramCell::new());
        Self {
            cells,
            count_max: 0,
        }
    }

    pub fn width(&self) -> usize {
        self.cells.ncols()
    }

    pub fn height(&self) -> usize {
        self.cells.nrows()
    }

    pub fn count_max(&self) -> u64 {
        self.count_max
    }

    pub fn count_max_mut(&mut self) -> &mut u64 {
        &mut self.count_max
    }

    #[inline(always)]
    pub fn get(&self, x: usize, y: usize) -> &HistogramCell {
        &self.cells[(y, x)]
    }

    #[inline(always)]
    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut HistogramCell {
        &mut self.cells[(y, x)]
    }

    pub fn clear(&mut self) {
        self.cells.fill(HistogramCell::new());
        self.count_max = 0;
    }
}
