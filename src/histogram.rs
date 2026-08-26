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
    cells: Array2<HistogramCell>,
}

impl Histogram {
    pub fn new(width: usize, height: usize, color: f64) -> Self {
        // MxN, M rows, N columns
        let cells = Array2::from_shape_fn((height, width), |(_i, _j)| HistogramCell::new(color));
        Self { cells }
    }

    pub fn width(&self) -> usize {
        self.cells.ncols()
    }

    pub fn height(&self) -> usize {
        self.cells.nrows()
    }
}
