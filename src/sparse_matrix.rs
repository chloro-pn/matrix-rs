use std::ops::Add;
use std::ops::Mul;
use std::fmt::{Formatter, Display};

use crate::matrix_base::*;

#[derive(Clone, Copy)]
struct Item<T> {
    index : i64,
    value : T,
}

impl<T> Item<T> {
    fn new(i : i64, v : T) -> Item<T> {
        Item {
            index : i,
            value : v,
        }
    }
}

impl<T : Mul<Output = T>> Mul<T> for Item<T> {
    type Output = Item<T>;
    fn mul(self : Self, other : T) -> Self::Output {
        Item {
            index : self.index,
            value : self.value * other,
        }
    }
}

impl<T : Add<Output = T>> Add for Item<T> {
    type Output = Item<T>;
    fn add(self : Self, other : Self) -> Self::Output {
        Item {
            index : self.index,
            value : self.value + other.value,
        }
    }
}
#[derive(Clone, Default)]
struct TheRow<T>(i64, Vec<Item<T>>);

impl<T : Default + Clone> TheRow<T> {
    fn new(row_index : i64) -> TheRow<T> {
        TheRow(row_index, Default::default())
    }
}

pub struct IterItem<'a, T>(i64, &'a Item<T>);

impl<'a, T> IterItem<'a, T> {
    fn get_row(self : &Self) -> i64 {
        self.0
    }

    fn get_col(self : &Self) -> i64 {
        self.1.index
    }

    fn get_v(self : &Self) -> &T {
        &self.1.value
    }
}

pub struct RowIterator<'a, T> {
    row : i64,
    real_index : i64,
    holder : &'a Vec<Item<T>>,
}

impl<'a, T> RowIterator<'a, T> {
    fn new(row : i64, h : &'a Vec<Item<T>>) -> RowIterator<'a, T> {
        RowIterator {
            row : row,
            real_index : 0,
            holder : h,
        }
    }
}

impl<'a, T> Iterator for RowIterator<'a, T> {
    type Item = IterItem<'a, T>;
    fn next(self : &mut Self) -> Option<Self::Item> {
        if self.real_index as usize == self.holder.len() {
            None
        } else {
            let v = IterItem(self.row, &self.holder[self.real_index as usize]);
            self.real_index += 1;
            Some(v)
        }
    }
}

#[derive(Clone)]
pub struct SparseMatrix<T> {
    row : i64,
    col : i64,
    container : Vec<TheRow<T>>,
}

impl<T : Default + Clone> MatrixInit<T> for SparseMatrix<T> {
    fn new(row : &i64, col : &i64) -> Self {
        let mut m = SparseMatrix {
            row : *row,
            col : *col,
            container : Default::default(),
        };
        m.container.resize(*row as usize, TheRow::new(0));
        for row in 0..m.container.len() {
            m.container[row].0 = row as i64;
        }
        m
    }
}

impl<T : Display + Clone + Default> ConstMatrix<T> for SparseMatrix<T> {
    fn get_row(self : &Self) -> i64 {
        self.row
    }

    fn get_column(self : &Self) -> i64 {
        self.col
    }

    fn get(self : &Self, row : &i64, col : &i64) -> Option<&T> {
        if *row >= self.get_row() || *col >= self.get_column() {
            panic!("out of range");
        }
        let the_row : &TheRow<T> = &self.container[*row as usize];
        let the_item = the_row.1.iter().find(|x| x.index == *col);
        match the_item {
            Some(item) => Some(&item.value),
            None => None,
        }
    }

    fn get_sub_matrix(&self, row_begin : &i64, row : &i64, col_begin : &i64, col : &i64) -> Self {
        let mut m = Self::new(row, col);
        for i in *row_begin..(*row_begin + *row) {
            let m_i = i - *row_begin;
            for each in self.get_iterator(&i) {
                if each.get_col() >= *col_begin && each.get_col() < *col_begin + *col {
                    m.container[m_i as usize].1.push(Item::new(each.get_col() - *col_begin, each.get_v().clone()));
                }
            }
        }
        m
    }
}

impl<T : Display> Display for SparseMatrix<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "smatrix[{}, {}] : \n", self.row, self.col).unwrap();
        for row in 0..self.row {
            let iterator = self.get_iterator(&row);
            for v in iterator {
                write!(f, "[{}, {}] = {} ", v.get_row(), v.get_col(), v.get_v()).unwrap();
            }
            write!(f, "\n").unwrap();
        }
        write!(f, "")
    }
}

impl<'a, T> MatrixIterator<'a, RowIterator<'a, T>> for SparseMatrix<T> {
    fn get_iterator<'b : 'a>(self : &'b Self, row : &i64) -> RowIterator<'a, T> {
        RowIterator::new(*row, &self.container[*row as usize].1)
    }
}

impl<T : Group<T> + Default + Clone + Copy + Add<Output = T> + Mul<Output = T> + Display + PartialEq> Matrix<T> for SparseMatrix<T> {
    fn set(self : &mut Self, row : &i64, col : &i64, value : T) {
        if *row >= self.get_row() || *col >= self.get_column() {
            panic!("out of range");
        }
        let the_row  = &mut self.container[*row as usize].1;
        let the_item = the_row.iter_mut().find(|x| x.index == *col);
        if let Some(mut item) = the_item {
            item.value = value;
            return;
        }
        the_row.push(Item::new(*col, value));
        if the_row.len() > 1 {
            the_row.sort_by(|a, b| a.index.cmp(&b.index));
        } 
    }

    fn add(self : &mut Self, row : &i64, col : &i64, value : T) {
        if *row >= self.get_row() || *col >= self.get_column() {
            panic!("out of range");
        }
        let the_row = &mut self.container[*row as usize].1;
        let the_item = the_row.iter_mut().find(|x | x.index == *col);
        if let Some(mut item) = the_item {
            item.value = item.value + value;
            return;
        }
        the_row.push(Item::new(*col, value));
        if the_row.len() > 1 {
            the_row.sort_by(|a, b| a.index.cmp(&b.index));
        } 
    }

    fn element_row_transform_swap(&mut self, row_i : &i64, row_j : &i64) {
        self.container.swap(*row_i as usize, *row_j as usize);
        self.container[*row_i as usize].0 = *row_i;
        self.container[*row_j as usize].0 = *row_j;
    }

    fn element_row_transform_multi(&mut self, row : &i64, k : T) {
        let the_row = &mut self.container[*row as usize];
        for each in the_row.1.iter_mut() {
            each.value = each.value * k;
        }
    }

    fn element_row_transform_plus(&mut self, row_i : &i64, row_j : &i64, k : T) {
        let the_row_i = &self.container[*row_i as usize].1;
        let the_row_j = &self.container[*row_j as usize].1;
        if the_row_j.len() == 0 {
            return;
        } else if the_row_i.len() == 0 {
            let the_row_i : Vec<_> = the_row_j.clone()
                                              .into_iter()
                                              .map(|x| x * k)
                                              .collect();
            self.container[*row_i as usize] = TheRow(*row_i, the_row_i);
            return;
        } else {
            let mut tmp : Vec<Item<T>> = Vec::new();
            let mut real_index_i = 0 as usize;
            let mut real_index_j = 0 as usize;
            loop {
                let col_index_i = the_row_i[real_index_i].index;
                let col_index_j = the_row_j[real_index_j].index;
                if col_index_i == col_index_j {
                    tmp.push(the_row_i[real_index_i] + the_row_j[real_index_j] * k);
                    real_index_i += 1;
                    real_index_j += 1;
                } else if col_index_i < col_index_j {
                    tmp.push(the_row_i[real_index_i]);
                    real_index_i += 1;
                } else {
                    tmp.push(the_row_j[real_index_j] * k);
                    real_index_j += 1;
                }
                if real_index_i >= the_row_i.len() || real_index_j >= the_row_j.len() {
                    break;
                }
            }
            while real_index_i < the_row_i.len() {
                tmp.push(the_row_i[real_index_i]
                );
                real_index_i += 1;
            }
            while real_index_j < the_row_j.len() {
                tmp.push(the_row_j[real_index_j] * k);
                //tmp.push(Item::new(the_row_j[real_index_j].index, the_row_j[real_index_j].value * k));
                real_index_j += 1;
            }
            self.container[*row_i as usize] = TheRow(*row_i, tmp);   
        }
    }
}