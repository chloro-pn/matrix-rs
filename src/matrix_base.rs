use std::ops::Mul;
use std::cmp::PartialEq;
pub trait Group<T : Mul<T, Output = T>> {
    fn get_inverse_mul(self : &Self) -> T;
    fn get_identity_mul() -> T;
    fn get_identity_add() -> T;
    fn get_inverse_add(self : &Self) -> T;
}

impl Group<f32> for f32 {
    fn get_identity_mul() -> f32 {
        1.0
    }

    fn get_inverse_mul(self : &Self) -> f32 {
        Self::get_identity_mul() / *self
    }

    fn get_identity_add() -> f32 {
        0.0
    }

    fn get_inverse_add(self : &Self) -> f32 {
        Self::get_identity_add() - *self
    }
}

pub trait MatrixInit<T> {
    fn new(row : usize, col : usize) -> Self;
}

pub trait ConstMatrix<T> {
    fn get_row(&self) -> usize;
    fn get_column(&self) ->usize;
    fn get(&self, row : usize, col : usize) -> Option<&T>;
    fn get_sub_matrix(&self, row_begin : usize, row : usize, col_begin : usize, col : usize) -> Self;
}

pub trait MatrixIterator<'a, T : Iterator + 'a> {
    fn get_iterator<'b : 'a>(self : &'b Self, row : usize) -> T;
}

pub trait Matrix<T : Mul<Output = T> + Group<T> + PartialEq + Clone> : MatrixInit<T> + ConstMatrix<T> + Clone + Sized {
    fn set(&mut self, row : usize, col : usize, value : T);
    fn add(&mut self, row : usize, col : usize, value : T);
    fn element_row_transform_swap(&mut self, row_i : usize, row_j : usize);
    fn element_row_transform_multi(&mut self, row : usize, k : T);
    fn element_row_transform_plus(&mut self, row_i : usize, row_j : usize, k : T);

    fn set_from_matrix<T2 : ConstMatrix<T>>(self : &mut Self, row_begin : usize, col_begin : usize, m : &T2) {
        for i in 0..m.get_row() {
            for j in 0..m.get_column() {
                let s_i = i + row_begin;
                let s_j = j + col_begin;
                self.set(s_i, s_j, m.get(i, j).unwrap().clone());
            } 
        }
    }

    fn get_identity_matrix(rc : usize) -> Self {
        let mut m = Self::new(rc, rc);
        for i in 0..rc {
            m.set(i, i, T::get_identity_mul() /*单位元*/);
        }
        m
    }

    fn inverse(self : &Self) -> Option<Self> {
        if self.get_row() != self.get_column() {
            panic!("matrix inverse need row == col");
        }
        if self.get_row() == 1 && self.get_column() == 1 {
            let mut m = self.clone();
            m.set(0, 0, (*self.get(0, 0).unwrap()).get_inverse_mul() /*逆元*/);
            return Some(m);
        }
        let mut myself = self.clone();
        let mut result = Self::get_identity_matrix(self.get_row());
        for i in 0..self.get_row() {
            if *myself.get(i, i).unwrap() == T::get_identity_add() {
                let mut error = true;
                for j in (i+1)..self.get_row() {
                    if *self.get(j, i).unwrap() != T::get_identity_add() {
                        myself.element_row_transform_swap(i, j);
                        result.element_row_transform_swap(i, j);
                        error = false;
                        break;
                    }
                }
                if error == true {
                    return None;
                }
            }
            for j in (i+1)..self.get_row() {
                let j_i = myself.get(j, i).unwrap();
                if *j_i != T::get_identity_add() {
                    let k = ((*j_i).clone() * myself.get(i, i).unwrap().get_inverse_mul()).get_inverse_add();
                    myself.element_row_transform_plus(j, i, k.clone());
                    result.element_row_transform_plus(j, i, k.clone());
                }
            }
            let k = myself.get(i, i).unwrap().get_inverse_mul();
            myself.element_row_transform_multi(i, k.clone());
            result.element_row_transform_multi(i, k.clone());
        }
        for i in 1..self.get_row() {
            let mut j : i64 = (i - 1) as i64;
            while j >= 0 {
                let k = myself.get(j as usize, i).unwrap().get_inverse_add();
                myself.element_row_transform_plus(j as usize, i, k.clone());
                result.element_row_transform_plus(j as usize, i, k.clone());
                j -= 1;
            }
        }
        Some(result)
    }
}

#[macro_export]
macro_rules! matrix_row {
    (($($var : expr),+)) => {{
        let mut v = Vec::new();
        $(v.push($var);)+
        v   
    }};
}

#[macro_export]
macro_rules! matrix {
    ($mtype : ty => $($row : tt);+) => {{
        let mut vc = Vec::new();
        let mut r : usize = 0;
        let mut c : usize = 0;
        $(
        let v = matrix_row!($row);
        if r == 0 {
            c = v.len();
        } else {
            if c != v.len() {
                panic!("matrix row has different len, {} != {}", c, v.len());
            }
        }
        vc.push(v);
        r += 1;
        )+
        let mut m = <$mtype>::new(r, c);
        r = 0;
        for each_row in vc.into_iter() {
            c = 0;
            for each in each_row.into_iter() {
                m.set(r, c, each);
                c += 1;
            }
            r += 1;
        }
        m
    }};
}

