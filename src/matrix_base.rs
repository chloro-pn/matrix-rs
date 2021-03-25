pub trait Matrix<T> {
    fn get_row(&self) -> i64;
    fn get_column(&self) ->i64;
    fn set(&mut self, row : &i64, col : &i64, value : T);
    fn add(&mut self, row : &i64, col : &i64, value : T);
    fn get(&self, row : &i64, col : &i64) -> Option<&T>;
    fn element_row_transform_swap(&mut self, row_i : &i64, row_j : &i64);
    fn element_row_transform_multi(&mut self, row : &i64, k : T);
    fn element_row_transform_plus(&mut self, row_i : &i64, row_j : &i64, k : T);
    fn print(self : &Self);
}