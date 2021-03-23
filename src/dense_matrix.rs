use std::ops::Add;
use std::ops::Mul;
use std::fmt::Display;

use crate::matrix_base;

pub struct DenseMatrix<T : Default + Copy> {
    row : i64,
    col : i64,
    container : Vec<T>,
}

impl<T : Default + Copy> DenseMatrix<T> {
    pub fn new(r : i64, c : i64) -> DenseMatrix<T> {
        let mut v = DenseMatrix {
            row : r,
            col : c,
            container : Vec::new(),
        };
        v.container.resize((r * c) as usize, Default::default());
        v
    }

    pub fn get_index(self : &Self, r : &i64, c : &i64) -> usize {
        (*r * self.col + *c) as usize
    }
}

impl<T : Default + Copy + Add<Output = T> + Mul<Output = T> + Display> matrix_base::Matrix<T> for DenseMatrix<T> {
    fn get_row(self : &Self) -> i64 {
        return self.row;
    }

    fn get_column(self : &Self) -> i64 {
        return self.col;
    }

    fn set(self : &mut Self, row : &i64, col : &i64, value : T) {
        if *row >= self.get_row() || *col >= self.get_column() {
            panic!("out of range");
        }
        let index = self.get_index(row, col);
        self.container[index] = value;
    }

    fn add(self : &mut Self, row : &i64, col : &i64, value : T) {
        if *row >= self.get_row() || *col >= self.get_column() {
            panic!("out of range");
        }
        let index = self.get_index(row, col);
        self.container[index] = value + self.container[index];
    }

    fn get<'a>(self : &'a Self, row : &i64, col : &i64) -> Option<&'a T> {
        if *row >= self.get_row() || *col >= self.get_column() {
            panic!("out of range");
        }
        Some(&self.container[self.get_index(row, col)])
    }

    fn element_row_transform_swap(&mut self, row_i : &i64, row_j : &i64) {
        if *row_i >= self.get_row() || *row_j >= self.get_row() {
            panic!("out of range");
        }
        for j in 0..self.get_column() {
            let i1 = self.get_index(&row_i, &j);
            let i2 = self.get_index(&row_j, &j);
            self.container.swap(i1, i2);
        }
    }

    fn element_row_transform_multi(&mut self, row : &i64, k : T) {
        if *row >= self.get_row() {
            panic!("out of range");
        }
        for j in 0..self.get_column() {
            let index = self.get_index(&row, &j);
            self.container[index] = self.container[index] * k;
        }
    }

    fn element_row_transform_plus(&mut self, row_i : &i64, row_j : &i64, k : T) {
        if *row_i >= self.get_row() || *row_j >= self.get_row() {
            panic!("out of range");
        }
        for j in 0..self.get_column() {
            let i1 = self.get_index(&row_i, &j);
            let i2 = self.get_index(&row_j, &j);
            self.container[i1] = self.container[i1] + self.container[i2] * k;
        }
    }

    fn print(self : &Self) {
        println!("matrix row = {}, col = {}", self.get_row(), self.get_column());
        for i in 0..self.get_row() {
            for j in 0..self.get_column() {
                println!("m[{}, {}] = {}", i, j, *self.get(&i, &j).unwrap());
            }
        }
    }
}
