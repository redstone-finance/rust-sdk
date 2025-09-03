use alloc::vec::Vec;

use core::iter::Iterator;

pub struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    columns: usize,
}

impl<T: Default + Copy> Matrix<T> {
    pub fn new(rows: usize, columns: usize) -> Self {
        Self {
            data: vec![Default::default(); rows * columns],
            rows,
            columns,
        }
    }
}

impl<T: Copy> Matrix<T> {
    pub fn mut_unchecked_at(&mut self, row: usize, column: usize) -> &mut T {
        &mut self.data[row * self.columns + column]
    }

    pub fn rows_iter(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        (0..self.rows).map(|row| {
            let start = row * self.columns;
            let end = start + self.columns;

            self.data[start..end].iter()
        })
    }
}
