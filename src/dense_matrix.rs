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

    fn get_v(self : &Self) -> &T {
        self.2
    }
}

pub struct RowIterator<'a, T> {
    row : i64,
    col : i64,
    holder : &'a DenseMatrix<T>,
}

impl<'a, T> RowIterator<'a, T> {
    fn new(r : i64, h : &'a DenseMatrix<T>) -> RowIterator<'a, T> {
        RowIterator {
            row : r,
            col : 0,
            holder : h
        }
    }
}

impl<'a, T> Iterator for RowIterator<'a, T> {
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

#[derive(Clone)]
pub struct DenseMatrix<T> {
    row : i64,
    col : i64,
    container : Vec<T>,
}

impl<T : Default + Clone> MatrixInit<T> for DenseMatrix<T> {
    fn new(row : &i64, col : &i64) -> Self {
        let mut m = DenseMatrix {
            row : *row,
            col : *col,
            container : Default::default(),
        };
        m.container.resize((*row * *col) as usize, Default::default());
        m
    }
}

impl<T : Display + Default + Copy> ConstMatrix<T> for DenseMatrix<T> {
    fn get_row(self : &Self) -> i64 {
        return self.row;
    }

    fn get_column(self : &Self) -> i64 {
        return self.col;
    }

    fn get(self : &Self, row : &i64, col : &i64) -> Option<&T> {
        if *row >= self.get_row() || *col >= self.get_column() {
            panic!("out of range");
        }
        Some(&self.container[self.get_index(row, col)])
    }

    fn get_sub_matrix(&self, row_begin : &i64, row : &i64, col_begin : &i64, col : &i64) -> DenseMatrix<T> {
        let mut m : DenseMatrix<T> = DenseMatrix::new(row, col);
        for i in *row_begin..(*row_begin + *row) {
            for j in *col_begin..(*col_begin + *col) {
                let m_i = i - *row_begin;
                let m_j = j - *col_begin;
                let index = m.get_index(&m_i, &m_j);
                m.container[index] = *self.get(&i, &j).unwrap();
            }
        }
        m
    }

    fn print(self : &Self) {
        println!("matrix row = {}, col = {}", self.get_row(), self.get_column());
        for row in 0..self.row {
            let iterator = self.get_iterator(&row);
            for v in iterator {
                print!("[{}, {}] = {}", v.get_row(), v.get_col(), v.get_v());
            }
            print!("\n");
        }
    }
}

impl<T> DenseMatrix<T> {
    pub fn get_index(self : &Self, r : &i64, c : &i64) -> usize {
        (*r * self.col + *c) as usize
    }
}

impl<'a, T> MatrixIterator<'a, RowIterator<'a, T>> for DenseMatrix<T> {
    fn get_iterator<'b : 'a>(self : &'b Self, row : &i64) -> RowIterator<'a, T> {
        RowIterator::new(*row, self)
    }
}

impl<T : Clone> DenseMatrix<T> {
    pub fn set_nth_column(self : &mut Self, col : i64, v : Vector<T>) {
        if v.length() != self.row as usize {
            panic!("set column error, mismatch length !");
        }
        for i in 0..self.row {
            let index = self.get_index(&i, &col);
            self.container[index] = v[i as usize].clone();
        }
    }
}

impl<T : Clone + Default> DenseMatrix<T> {
    pub fn get_nth_column(self : &mut Self, col : i64) -> Vector<T> {
        let mut v = Vector::new(self.row);
        for i in 0..self.row {
            let index = self.get_index(&i, &col);
            v.set(i, self.container[index].clone());
        }
        v
    }
}

impl<T> Matrix<T> for DenseMatrix<T>
    where T: Default + Copy + Add<Output = T> + Mul<Output = T> + Display + Group<T> + PartialEq {
    fn set(self : &mut Self, row : &i64, col : &i64, value : T) {
        let index = self.get_index(row, col);
        self.container[index] = value;
    }

    fn add(self : &mut Self, row : &i64, col : &i64, value : T) {
        let index = self.get_index(row, col);
        self.container[index] = value + self.container[index];
    }

    fn element_row_transform_swap(&mut self, row_i : &i64, row_j : &i64) {
        for j in 0..self.get_column() {
            let i1 = self.get_index(&row_i, &j);
            let i2 = self.get_index(&row_j, &j);
            self.container.swap(i1, i2);
        }
    }

    fn element_row_transform_multi(&mut self, row : &i64, k : T) {
        for j in 0..self.get_column() {
            let index = self.get_index(&row, &j);
            self.container[index] = self.container[index] * k;
        }
    }

    fn element_row_transform_plus(&mut self, row_i : &i64, row_j : &i64, k : T) {
        for j in 0..self.get_column() {
            let i1 = self.get_index(&row_i, &j);
            let i2 = self.get_index(&row_j, &j);
            self.container[i1] = self.container[i1] + self.container[i2] * k;
        }
    }
}
