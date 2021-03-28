use std::ops::Add;
use std::ops::Mul;
use std::fmt::{Formatter, Display};

use crate::matrix_base::*;
use crate::vector::*;

pub struct Item<'a, T>(usize, usize, &'a T);

impl<'a, T> Item<'a, T> {
    fn new(row : usize, col : usize, v : &'a T) -> Item<'a, T> {
        Item(row, col, v)
    }

    fn get_row(self : &Self) -> usize {
        self.0
    }

    fn get_col(self : &Self) -> usize {
        self.1
    }

    fn get_v(self : &Self) -> &T {
        self.2
    }
}

pub struct RowIterator<'a, T> {
    row : usize,
    col : usize,
    holder : &'a DenseMatrix<T>,
}

impl<'a, T> RowIterator<'a, T> {
    fn new(r : usize, h : &'a DenseMatrix<T>) -> RowIterator<'a, T> {
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
            let v = &self.holder.container[self.holder.get_index(self.row, self.col)];
            let tmp = Some(Item::new(self.row, self.col, v));
            self.col += 1;
            tmp
        }
    }
}

#[derive(Clone)]
pub struct DenseMatrix<T> {
    row : usize,
    col : usize,
    container : Vec<T>,
}

impl<T : Default + Clone> MatrixInit<T> for DenseMatrix<T> {
    fn new(row : usize, col : usize) -> Self {
        let mut m = DenseMatrix {
            row : row,
            col : col,
            container : Default::default(),
        };
        m.container.resize(row * col, Default::default());
        m
    }
}

impl<T : Default + Copy> ConstMatrix<T> for DenseMatrix<T> {
    fn get_row(self : &Self) -> usize {
        return self.row;
    }

    fn get_column(self : &Self) -> usize {
        return self.col;
    }

    fn get(self : &Self, row : usize, col : usize) -> Option<&T> {
        if row >= self.get_row() || col >= self.get_column() {
            panic!("out of range");
        }
        Some(&self.container[self.get_index(row, col)])
    }

    fn get_sub_matrix(&self, row_begin : usize, row : usize, col_begin : usize, col : usize) -> DenseMatrix<T> {
        let mut m : DenseMatrix<T> = DenseMatrix::new(row, col);
        for i in row_begin..(row_begin + row) {
            for j in col_begin..(col_begin + col) {
                let m_i = i - row_begin;
                let m_j = j - col_begin;
                let index = m.get_index(m_i, m_j);
                m.container[index] = *self.get(i, j).unwrap();
            }
        }
        m
    }
}

impl<T : Display> Display for DenseMatrix<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "dmatrix[{}, {}]:\n", self.row, self.col).unwrap();
        for row in 0..self.row {
            let iterator = self.get_iterator(row);
            for v in iterator {
                write!(f, "[{}, {}] = {} ", v.get_row(), v.get_col(), v.get_v()).unwrap();
            }
            write!(f, "\n").unwrap();
        }
        write!(f, "")
    }
}

impl<T> DenseMatrix<T> {
    pub fn get_index(self : &Self, r : usize, c : usize) -> usize {
        r * self.col + c
    }
}

impl<'a, T> MatrixIterator<'a, RowIterator<'a, T>> for DenseMatrix<T> {
    fn get_iterator<'b : 'a>(self : &'b Self, row : usize) -> RowIterator<'a, T> {
        RowIterator::new(row, self)
    }
}

impl<T : Clone> DenseMatrix<T> {
    #[warn(dead_code)]
    pub fn set_nth_column(self : &mut Self, col : usize, v : Vector<T>) {
        if v.length() != self.row {
            panic!("set column error, mismatch length !");
        }
        for i in 0..self.row {
            let index = self.get_index(i, col);
            self.container[index] = v[i].clone();
        }
    }
}

impl<T : Clone + Default> DenseMatrix<T> {
    pub fn get_nth_column(self : &Self, col : usize) -> Vector<T> {
        let mut v = Vector::new(self.row);
        for i in 0..self.row {
            let index = self.get_index(i, col);
            v.set(i, self.container[index].clone());
        }
        v
    }
}

impl<T> Matrix<T> for DenseMatrix<T>
    where T: Default + Copy + Add<Output = T> + Mul<Output = T> + Display + Group<T> + PartialEq {
    fn set(self : &mut Self, row : usize, col : usize, value : T) {
        let index = self.get_index(row, col);
        self.container[index] = value;
    }

    fn add(self : &mut Self, row : usize, col : usize, value : T) {
        let index = self.get_index(row, col);
        self.container[index] = value + self.container[index];
    }

    fn element_row_transform_swap(&mut self, row_i : usize, row_j : usize) {
        for j in 0..self.get_column() {
            let i1 = self.get_index(row_i, j);
            let i2 = self.get_index(row_j, j);
            self.container.swap(i1, i2);
        }
    }

    fn element_row_transform_multi(&mut self, row : usize, k : T) {
        for j in 0..self.get_column() {
            let index = self.get_index(row, j);
            self.container[index] = self.container[index] * k;
        }
    }

    fn element_row_transform_plus(&mut self, row_i : usize, row_j : usize, k : T) {
        for j in 0..self.get_column() {
            let i1 = self.get_index(row_i, j);
            let i2 = self.get_index(row_j, j);
            self.container[i1] = self.container[i1] + self.container[i2] * k;
        }
    }
}