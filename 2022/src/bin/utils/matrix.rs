use super::vector::Vector;

pub struct Matrix<T> {
  pub data: Vec<Vec<T>>
}

impl<T> Matrix<T> {
  pub fn new(data: Vec<Vec<T>>) -> Self {
    Matrix { data }
  }

  pub fn width(&self) -> usize {
    self.data[0].len()
  }

  pub fn height(&self) -> usize {
    self.data.len()
  }

  pub fn get(&self, pos: &Vector) -> &T {
    &self.data[pos.1 as usize][pos.0 as usize]
  }

  pub fn set(&mut self, pos: &Vector, value: T) {
    self.data[pos.1 as usize][pos.0 as usize] = value;
  }

  pub fn is_in_bounds(&self, pos: &Vector) -> bool {
    pos.0 >= 0 && pos.0 < self.width() as i32
    && pos.1 >= 0 && pos.1 < self.height() as i32
  }
}