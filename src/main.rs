
mod dense_matrix;
mod sparse_matrix;
mod matrix_base;
mod vector;

use dense_matrix::DenseMatrix;
use sparse_matrix::SparseMatrix;
use matrix_base::*;
use vector::Vector;

fn main() {
    let mut m : DenseMatrix<f32> = DenseMatrix::new(&3, &3);
    m.set(&0, &0, 1.0);
    m.set(&0, &1, 2.0);
    m.set(&0, &2, 3.0);
    m.set(&1, &0, 2.0);
    m.set(&1, &1, 2.0);
    m.set(&1, &2, 1.0);
    m.set(&2, &0, 3.0);
    m.set(&2, &1, 4.0);
    m.set(&2, &2, 3.0);
    m.inverse().print();
    m.inverse().get_sub_matrix(&1,&2,&1,&2).print();

    let mut m : SparseMatrix<f32> = SparseMatrix::new(&200, &500);
    m.set(&105, &25, 143.0);
    m.add(&105, &25, 2.0);
    m.set(&2, &24, 15.0);
    m.element_row_transform_swap(&105, &2);
    m.element_row_transform_plus(&2, &105, 2.0);
}