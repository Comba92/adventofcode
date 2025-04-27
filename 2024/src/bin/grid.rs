use std::ops::{Index, IndexMut};

struct Grid<T: Default + Clone> {
  data: Box<[T]>,
  pub width: usize,
  pub height: usize,
}

type Coordinate = (isize, isize);

impl<T: Default + Clone> Grid<T> {
  pub fn new(width: usize, height: usize) -> Self {
    Self {
      data: vec![T::default(); width * height].into_boxed_slice(),
      width, height
    }
  }

  fn is_coord_in_bounds(&self, (x, y): Coordinate) -> bool {
    x >= 0 && y >= 0 && x < self.width as isize && y < self.height as isize
  }

  pub fn get(&self, pos: Coordinate) -> Option<&T> {
    if self.is_coord_in_bounds(pos) {
      Some(&self[pos])
    } else {
      None
    }
  }

  pub fn get_mut(&mut self, pos: Coordinate) -> Option<&mut T> {
    if self.is_coord_in_bounds(pos) {
      Some(&mut self[pos])
    } else {
      None
    }
  }

  pub fn iter(&self) -> std::slice::Iter<T> {
    self.data.iter()
  }
}

impl<T: Default + Clone> Index<Coordinate> for Grid<T> {
  type Output = T;

  fn index(&self, index: Coordinate) -> &Self::Output {
    let idx = index.1 * self.width as isize + index.0;
    &self.data[idx as usize]
  }
}

impl<T: Default + Clone> IndexMut<Coordinate> for Grid<T> {
  fn index_mut(&mut self, index: Coordinate) -> &mut Self::Output {
    let idx = index.1 * self.width as isize + index.0;
    &mut self.data[idx as usize]
  }
}

