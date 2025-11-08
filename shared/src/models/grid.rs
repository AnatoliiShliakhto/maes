use crate::models::QuizGrade;
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
    
    pub fn add_value(&mut self, row: usize, col: usize, val: T)
    where
        T: std::ops::AddAssign + Copy + 'static,
    {
        let idx = self.idx(row, col);
        self.data[idx] += val;
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

    pub fn set_row(&mut self, row: usize, data: Vec<T>) {
        for (col, val) in data.into_iter().enumerate() {
            self.set(row, col, val);
        }
    }

    pub fn fill(&mut self, val: T) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                self.set(row, col, val.clone());
            }
        }
    }

    pub fn fill_row(&mut self, row: usize, val: T) {
        for col in 0..self.cols {
            self.set(row, col, val.clone());
        }
    }

    pub fn concat(&mut self, other: &Self)
    where
        T: std::ops::Add<Output = T> + Copy + 'static,
    {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let sum = *self.get(row, col) + *other.get(row, col);
                self.set(row, col, sum);
            }
        }
    }

    pub fn calc_col_average(&self, col: usize) -> f64
    where
        T: Copy + Into<usize>,
    {
        let mut sum = 0_usize;
        for row in 0..self.rows {
            sum += (*self.get(row, col)).into();
        }
        sum as f64 / self.rows as f64
    }

    pub fn calc_row_average(&self, row: usize) -> f64
    where
        T: Copy + Into<usize>,
    {
        let mut sum = 0_usize;
        for col in 0..self.cols {
            sum += (*self.get(row, col)).into();
        }
        sum as f64 / self.cols as f64
    }

    pub fn calc_col_average_grade(&self, col: usize, grade: &QuizGrade) -> f64
    where
        T: Copy + Into<usize>,
    {
        let mut sum = 0_usize;
        for row in 0..self.rows {
            sum += grade.calc((*self.get(row, col)).into());
        }
        sum as f64 / self.rows as f64
    }

    pub fn calc_row_average_grade(&self, row: usize, grade: &QuizGrade) -> f64
    where
        T: Copy + Into<usize>,
    {
        let mut sum = 0_usize;
        for col in 0..self.cols {
            sum += grade.calc((*self.get(row, col)).into());
        }
        sum as f64 / self.cols as f64
    }
    
    pub fn extend_rows(&mut self, other: &Self) {
        if self.cols != other.cols {
            return;
        }
        self.rows += other.rows;
        self.data.extend(other.data.clone());
    }

    pub fn merge(&mut self, other: &Self)
    where
        T: std::ops::AddAssign + Copy + 'static,
    {
        if self.rows != other.rows || self.cols != other.cols {
            return;
        }

        for row in 0..self.rows {
            for col in 0..self.cols {
                self.add_value(row, col, *other.get(row, col));
            }
        }
    }
}
