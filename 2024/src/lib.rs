use std::ops::{Index, IndexMut};

pub type Coordinate = (isize, isize);

pub struct Vec2D {
  x: isize,
  y: isize,
}
impl Vec2D {
  pub fn new(x: isize, y: isize) -> Self {
    Self {x, y}
  }

  pub fn add(&self, other: &Self) -> Self {
    Self::new(self.x + other.x, self.y + other.y)
  }
}

impl From<(isize, isize)> for Vec2D {
  fn from(value: Coordinate) -> Self {
    Self::new(value.0, value.1)
  }
}

pub fn coord_add(a: Coordinate, b: Coordinate) -> Coordinate {
  (a.0 + b.0, a.1 + b.1)
}

#[derive(PartialEq, Eq)]
pub struct Grid<T> {
  data: Box<[T]>,
  pub width: usize,
  pub height: usize,
}

impl<T: std::fmt::Debug> std::fmt::Debug for Grid<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut res = f.debug_list();

    for item in self.data.iter().as_slice().chunks(self.width) {
      res.entry(&item);
    }

    res.finish()
  }
}

impl<T: Clone> Grid<T> {
  pub fn resize(width: usize, height: usize, value: T) -> Self {
    Self {
      data: vec![value; width * height].into_boxed_slice(),
      width, height
    }
  }
}

impl<T: Default + Clone> Grid<T> {
  pub fn reserve(width: usize, height: usize) -> Self {
    Self {
      data: vec![Default::default(); width * height].into_boxed_slice(),
      width, height
    }
  }

  pub fn new(width: usize, height: usize) -> Self {
    Self::reserve(width, height)
  }
}

impl<T> Grid<T> {
  pub fn pos_is_in_bounds(&self, (x, y): Coordinate) -> bool {
    x >= 0 && y >= 0 && x < self.width as isize && y < self.height as isize
  }

  pub fn get(&self, pos: Coordinate) -> Option<&T> {
    if self.pos_is_in_bounds(pos) {
      Some(&self[pos])
    } else {
      None
    }
  }

  pub fn get_mut(&mut self, pos: Coordinate) -> Option<&mut T> {
    if self.pos_is_in_bounds(pos) {
      Some(&mut self[pos])
    } else {
      None
    }
  }

  pub fn set(&mut self, pos: Coordinate, val: T) -> bool {
    if self.pos_is_in_bounds(pos) {
      self[pos] = val;
      true
    } else {
      false
    }
  }

  pub fn iter(&self) -> impl Iterator<Item = &T> {
    self.data.iter()
  }

  pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
    self.data.iter_mut()
  }

  pub fn iter_rows(&self) -> impl Iterator<Item = &[T]> {
    self.data.chunks_exact(self.width)
  }

  pub fn iter_rows_mut(&mut self) -> impl Iterator<Item = &mut [T]> {
    self.data.chunks_exact_mut(self.width)
  }

  pub fn iter_coords(&self) -> impl Iterator<Item = (Coordinate, &T)> {
    self.iter().enumerate().map(|(i, v)| 
      (((i % self.width) as isize, (i / self.height) as isize), v)
    )
  }

  pub fn iter_coords_mut(&mut self) -> impl Iterator<Item = (Coordinate, &mut T)> { 
    let width = self.width;
    let height = self.height;
    
    self.iter_mut().enumerate().map(move |(i, v)| {
      (((i % width) as isize, (i / height) as isize), v)
    })
  }
}

impl<T> Index<Coordinate> for Grid<T> {
  type Output = T;

  fn index(&self, (x, y): Coordinate) -> &Self::Output {
    let idx = y * self.width as isize + x;
    &self.data[idx as usize]
  }
}

impl<T> IndexMut<Coordinate> for Grid<T> {
  fn index_mut(&mut self, (x, y): Coordinate) -> &mut Self::Output {
    let idx = y * self.width as isize + x;
    &mut self.data[idx as usize]
  }
}

impl<T: Clone> From<&[&[T]]> for Grid<T> {
  fn from(value: &[&[T]]) -> Self {
    let width = value.first().map(|f| f.len()).unwrap_or_default();
    assert!(value.iter().all(|row| row.len() == width), "can't initialize grid: not all rows have a width equal to {width}");

    Self {
      data: value.concat().into_boxed_slice(),
      width,
      height: value.len(),
    }
  }
}

impl<T: Clone> From<&[Vec<T>]> for Grid<T> {
  fn from(value: &[Vec<T>]) -> Self {
    let width = value.first().map(|f| f.len()).unwrap_or_default();
    assert!(value.iter().all(|row| row.len() == width), "can't initialize grid: not all rows have a width equal to {width}");
    
    Self {
      data: value.concat().into_boxed_slice(),
      width,
      height: value.len(),
    }
  }
}

impl<T: Clone + Default> From<&mut [Vec<T>]> for Grid<T> {
  fn from(value: &mut [Vec<T>]) -> Self {
    let width = value.first().map(|f| f.len()).unwrap_or_default();
    assert!(value.iter().all(|row| row.len() <= width), "can't initialize grid: not all rows have a width equal or less than {width}");

    value.iter_mut()
      .for_each(|row| row.resize(width, T::default()));

    Self {
      data: value.concat().into_boxed_slice(),
      width,
      height: value.len(),
    }
  }
}

impl<T: Clone> From<&Vec<Vec<T>>> for Grid<T> {
  fn from(value: &Vec<Vec<T>>) -> Self {
    Self::from(value.as_slice())
  }
}

impl<T: Clone> From<&mut Vec<Vec<T>>> for Grid<T> {
  fn from(value: &mut Vec<Vec<T>>) -> Self {
    Self::from(value.as_slice())
  }
}

impl<T> IntoIterator for Grid<T> {
  type Item = T;
  type IntoIter = std::vec::IntoIter<T>;

  fn into_iter(self) -> Self::IntoIter {
    self.data.into_vec().into_iter()
  }
}

impl<T, I> FromIterator<I> for Grid<T>
where 
  I: Iterator<Item = T> + Clone,
{
  fn from_iter<F: IntoIterator<Item = I>>(iter: F) -> Self {
    let mut iter = iter.into_iter();

    let first_row = iter.next().unwrap();
    let width = first_row.clone().count();
    let mut data = Vec::from_iter(first_row);

    for row in iter {
      assert!(row.clone().count() == width, "can't initialize grid: not all rows have a width equal to {width}");
      data.extend(row);
    }

    let len = data.len();
    Self {
      data: data.into_boxed_slice(),
      width,
      height: len / width
    }
  }
}

// impl<T> FromIterator<Vec<T>> for Grid<T> {
//   fn from_iter<I: IntoIterator<Item = Vec<T>>>(iter: I) -> Self {
//     let mut iter = iter.into_iter();

//     let first_row = iter.next().unwrap_or_default();
//     let width = first_row.len();
//     let mut data = Vec::from(first_row);

//     for row in iter {
//       assert!(row.len() == width, "can't initialize grid: not all rows have a width equal to {width}");
//       data.extend(row);
//     }

//     let len = data.len();
//     Self {
//       data: data.into_boxed_slice(),
//       width,
//       height: len / width
//     }
//   }
// }