mod dense_matrix;
mod sparse_matrix;
mod matrix_base;
mod vector;

use dense_matrix::DenseMatrix;
use sparse_matrix::SparseMatrix;
use matrix_base::Matrix;
use vector::Vector;

fn main() {
    let mut m : DenseMatrix<i32> = DenseMatrix::new(3, 3);
    m.set(&1, &1, 25);
    m.set(&1, &2, 30);
    m.element_row_transform_multi(&1, 55);
    m.set_nth_column(0, Vector::new_with(3, 11));
    m.print();

    let mut m : SparseMatrix<i32> = SparseMatrix::new(200, 500);
    m.set(&105, &25, 143);
    m.add(&105, &25, 2);
    m.set(&2, &24, 15);
    m.element_row_transform_swap(&105, &2);
    m.element_row_transform_plus(&2, &105, 2);
    m.print();
}