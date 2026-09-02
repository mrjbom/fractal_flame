use ndarray::Array2;

#[derive(Clone)]
pub struct HistogramCell {
    pub count: u64,
    pub color: f64,
}

impl HistogramCell {
    pub fn new(color: f64) -> Self {
        Self { count: 0, color }
    }
}

#[derive(Clone)]
pub struct Histogram {
    // Use get and get_mut, or [(y, x)]
    pub cells: Array2<HistogramCell>,
}

impl Histogram {
    pub fn new(width: usize, height: usize) -> Self {
        // MxN, M rows, N columns
        let cells = Array2::from_shape_fn((height, width), |(_i, _j)| HistogramCell::new(0.0));
        Self { cells }
    }

    pub fn width(&self) -> usize {
        self.cells.ncols()
    }

    pub fn height(&self) -> usize {
        self.cells.nrows()
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
        self.cells.fill(HistogramCell::new(0.0));
    }
}
