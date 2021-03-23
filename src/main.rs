use std::ops::Add;
use std::ops::Mul;
use std::fmt::Display;

pub trait Matrix<T> {
    fn get_row(&self) -> i64;
    fn get_column(&self) ->i64;
    fn set(&mut self, row : &i64, col : &i64, value : T);
    fn add(&mut self, row : &i64, col : &i64, value : T);
    fn get<'a>(&'a self, row : &i64, col : &i64) -> Option<&'a T>;
    fn element_row_transform_swap(&mut self, row_i : &i64, row_j : &i64);
    fn element_row_transform_multi(&mut self, row : &i64, k : T);
    fn element_row_transform_plus(&mut self, row_i : &i64, row_j : &i64, k : T);
    fn print(self : &Self);
}

pub struct DenseMatrix<T : Default + Copy> {
    row : i64,
    col : i64,
    container : Vec<T>,
}

impl<T : Default + Copy> DenseMatrix<T> {
    fn new(r : i64, c : i64) -> DenseMatrix<T> {
        let mut v = DenseMatrix {
            row : r,
            col : c,
            container : Vec::new(),
        };
        v.container.resize((r * c) as usize, Default::default());
        v
    }

    fn get_index(self : &Self, r : &i64, c : &i64) -> usize {
        (*r * self.col + *c) as usize
    }
}

impl<T : Default + Copy + Add<Output = T> + Mul<Output = T> + Display> Matrix<T> for DenseMatrix<T> {
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

#[derive(Clone, Copy)]
pub struct Item<T> {
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

#[derive(Default, Clone)]
struct TheRow<T>(i64, Vec<Item<T>>);

impl<T : Default + Copy> TheRow<T> {
    fn new(row_index : i64) -> TheRow<T> {
        TheRow(row_index, Default::default())
    }
}

pub struct SparseMatrix<T : Default + Copy> {
    row : i64,
    col : i64,
    container : Vec<TheRow<T>>,
}

impl<T : Default + Copy> SparseMatrix<T> {
    fn new(row : i64, col : i64) -> SparseMatrix<T> {
        let mut m : SparseMatrix<T> = SparseMatrix {
            row : row,
            col : col,
            container : Default::default(),
        };
        m.container.resize(row as usize, Default::default());
        m
    }
}

impl<T : Default + Copy + Add<Output = T> + Mul<Output = T> + Display> Matrix<T> for SparseMatrix<T> {
    fn get_row(self : &Self) -> i64 {
        self.row
    }

    fn get_column(self : &Self) -> i64 {
        self.col
    }

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

    fn get<'a>(self : &'a Self, row : &i64, col : &i64) -> Option<&'a T> {
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
                    tmp.push(the_row_i[real_index_i].clone());
                    real_index_i += 1;
                } else {
                    tmp.push(Item::new(the_row_j[real_index_j].index, the_row_j[real_index_j].value * k));
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
                tmp.push(Item::new(the_row_j[real_index_j].index, the_row_j[real_index_j].value * k));
                real_index_j += 1;
            }
            self.container[*row_i as usize] = TheRow(*row_i, tmp);   
        }
    }

    fn print(self : &Self) {
        println!("matrix row = {}, col = {}", self.get_row(), self.get_column());
        for each in self.container.iter() {
            for v in each.1.iter() {
                println!("m[{}, {}] = {}", each.0, v.index, v.value);
            }
        }
    }
}

fn main() {
    let mut m : DenseMatrix<i32> = DenseMatrix::new(3, 3);
    m.set(&1, &1, 25);
    m.set(&1, &2, 30);
    println!("m[0, 0] = {}", m.get(&0, &0).unwrap());
    println!("m[1, 2] = {}", m.get(&1, &2).unwrap());
    m.element_row_transform_multi(&1, 55);
    m.print();

    let mut m : SparseMatrix<i32> = SparseMatrix::new(200, 500);
    m.set(&105, &25, 143);
    m.add(&105, &25, 2);
    m.set(&2, &24, 15);
    m.element_row_transform_swap(&105, &2);
    m.element_row_transform_plus(&2, &105, 2);
    m.print();
}