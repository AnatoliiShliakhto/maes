use ::serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct Grid<T: Clone + 'static> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T: Clone + 'static> Grid<T> {
    pub fn new(rows: usize, cols: usize, fill: T) -> Self {
        Self {
            rows,
            cols,
            data: vec![fill; rows * cols],
        }
    }

    pub fn idx(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.data[self.idx(row, col)]
    }

    pub fn set(&mut self, row: usize, col: usize, val: T) {
        let idx = self.idx(row, col);
        self.data[idx] = val;
    }

    pub fn get_col(&self, col: usize) -> Vec<&T> {
        let mut col_data = Vec::with_capacity(self.rows);
        for row in 0..self.rows {
            col_data.push(self.get(row, col));
        }
        col_data
    }

    pub fn get_row(&self, row: usize) -> Vec<&T> {
        let mut row_data = Vec::with_capacity(self.cols);
        for col in 0..self.cols {
            row_data.push(self.get(row, col));
        }
        row_data
    }
}
