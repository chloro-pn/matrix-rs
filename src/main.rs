
mod dense_matrix;
mod sparse_matrix;
mod matrix_base;
mod vector;

use dense_matrix::DenseMatrix;
use sparse_matrix::SparseMatrix;
use matrix_base::*;
use vector::Vector;

fn main() {
    let m = matrix![DenseMatrix<f32> => (1.0, 2.0, 3.0);(2.0, 2.0, 1.0);(3.0, 4.0, 3.0)];
    println!("{}", m.inverse().unwrap());

    let mut m : SparseMatrix<f32> = SparseMatrix::new(&200, &500);
    m.set(&105, &25, 143.0);
    m.add(&105, &25, 2.0);
    m.set(&2, &24, 15.0);
    m.element_row_transform_swap(&105, &2);
    m.element_row_transform_plus(&2, &105, 2.0);
}