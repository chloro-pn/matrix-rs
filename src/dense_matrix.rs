use std::ops::Add;
use std::ops::Mul;
use std::fmt::Display;

use crate::matrix_base::*;
use crate::vector::*;

pub struct Item<'a, T>(i64, i64, &'a T);

impl<'a, T> Item<'a, T> {
    fn new(row : i64, col : i64, v : &'a T) -> Item<'a, T> {
        Item(row, col, v)
    }

    fn get_row(self : &Self) -> i64 {
        self.0
    }

    fn get_col(self : &Self) -> i64 {
        self.1
    }

    fn get_v(self : &Self) -> &'_ T {
        self.2
    }
}

pub struct RowIterator<'a, T : Default + Copy> {
    row : i64,
    col : i64,
    holder : &'a DenseMatrix<T>,
}

impl<'a, T : Default + Copy> RowIterator<'a, T> {
    fn new(r : i64, h : &'a DenseMatrix<T>) -> RowIterator<'a, T> {
        RowIterator {
            row : r,
            col : 0,
            holder : h
        }
    }
}

impl<'a, T : Default + Copy> Iterator for RowIterator<'a, T> {
    type Item = Item<'a, T>;
    fn next(self : &mut Self) -> Option<Self::Item> {
        if self.col >= self.holder.col {
            None
        } else {
            let v = &self.holder.container[self.holder.get_index(&self.row, &self.col)];
            let tmp = Some(Item::new(self.row, self.col, v));
            self.col += 1;
            tmp
        }
    }
}

pub struct DenseMatrix<T : Default + Copy> {
    row : i64,
    col : i64,
    container : Vec<T>,
}

impl<'a, T : Default + Copy> DenseMatrix<T> {
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

    pub fn set_nth_column(self : &mut Self, col : i64, v : Vector<T>) {
        if v.length() != self.row as usize {
            panic!("set column error, mismatch length !");
        }
        for i in 0..self.row {
            let index = self.get_index(&i, &col);
            self.container[index] = v[i as usize];
        }
    }

    pub fn get_nth_column(self : &mut Self, col : i64) -> Vector<T> {
        let mut v = Vector::new(self.row);
        for i in 0..self.row {
            let index = self.get_index(&i, &col);
            v.set(i, self.container[index]);
        }
        v
    }
}

impl<'a, T : Default + Copy> MatrixIterator<'a, RowIterator<'a, T>> for DenseMatrix<T> {
    fn get_iterator(self : &'a Self, row : i64) -> RowIterator<'a, T> {
        RowIterator::new(row, self)
    }
}

impl<'a, T : Default + Copy + Add<Output = T> + Mul<Output = T> + Display> Matrix<'a, T, RowIterator<'a, T>> for DenseMatrix<T> {
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

    fn get(self : &Self, row : &i64, col : &i64) -> Option<&T> {
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
        for row in 0..self.row {
            let iterator = self.get_iterator(row);
            for v in iterator {
                println!("m[{}, {}] = {}", v.get_row(), v.get_col(), v.get_v());
            }
        }
    }
}
