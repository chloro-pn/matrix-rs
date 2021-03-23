mod dense_matrix;
mod sparse_matrix;
mod matrix_base;

use dense_matrix::DenseMatrix;
use sparse_matrix::SparseMatrix;
use matrix_base::Matrix;

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