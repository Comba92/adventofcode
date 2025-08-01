use std::ops::{Index, IndexMut};

pub type Coordinate = (isize, isize);
pub const DIRECTIONS: &[(isize, isize)] = &[
  (0, -1),
  (-1, 0),
  (1, 0),
  (0, 1),
];

pub const DIRECTIONS_DIAG: &[(isize, isize)] = &[
  (-1, -1),
  (0, -1),
  (1, -1),
  (-1, 0),
  (1, 0),
  (-1, 1),
  (0, 1),
  (1, 1),
];

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
  Up, Right, Down, Left, None
}

impl From<Coordinate> for Direction {
  fn from(value: Coordinate) -> Self {
    match value {
      (0, ..=-1) => Self::Left,
      (..=-1, 0) => Self::Up,
      (1.., 0) => Self::Down,
      (0, 1..) => Self::Right,
      _ => Self::None,
    }
  }
}

impl From<char> for Direction {
  fn from(value: char) -> Self {
    match value {
      '^' => Self::Up,
      '>' => Self::Right,
      'v' => Self::Down,
      '<' => Self::Left,
      _ => Self::None
    }
  }
}

impl From<u32> for Direction {
  fn from(value: u32) -> Self {
    match value {
      0 => Self::Up,
      1 => Self::Right,
      2 => Self::Down,
      3 => Self::Left,
      _ => Self::None,
    }
  }
}

impl Direction {
  pub fn to_vector(&self) -> Coordinate {
    match self {
      Self::Up    => (0, -1),
      Self::Right => (1, 0),
      Self::Down  => (0, 1),
      Self::Left  => (-1, 0),
      Self::None  => (0, 0),
    }
  }

  pub fn to_char(&self) -> char {
    match self {
      Self::Up => '^',
      Self::Right => '>',
      Self::Down => 'v',
      Self::Left => '<',
      Self::None => ' ',
    }
  }

  pub fn turn_left(&self) -> Direction {
    match self {
      Self::Up => Self::Left,
      Self::Down => Self::Right,
      Self::Left => Self::Down,
      Self::Right => Self::Up,
      Self::None => Self::None
    }
  }

  pub fn turn_right(&self) -> Direction {
    match self {
      Self::Up => Self::Right,
      Self::Down => Self::Left,
      Self::Left => Self::Up,
      Self::Right => Self::Down,
      Self::None => Self::None
    }
  }
}

pub fn coord_add(a: &Coordinate, b: &Coordinate) -> Coordinate {
  (a.0 + b.0, a.1 + b.1)
}
pub fn coord_sub(a: &Coordinate, b: &Coordinate) -> Coordinate {
  (a.0 - b.0, a.1 - b.1)
}
pub fn coord_mul(c: &Coordinate, s: isize) -> Coordinate {
  (c.0 * s, c.1 * s)
}


#[derive(PartialEq, Eq, Default, Clone)]
pub struct Grid<T: Default> {
  pub data: Vec<T>,
  pub width: usize,
  pub height: usize,
}

impl<T: std::fmt::Debug + Default> std::fmt::Debug for Grid<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut res = f.debug_list();

    for item in self.data.iter().as_slice().chunks(self.width) {
      res.entry(&item);
    }

    res.finish()
  }
}

impl<T: std::fmt::Display + Default> std::fmt::Display for Grid<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "\t{}", (0..self.width).map(|n| n.to_string()).collect::<String>())?;
    
    for (i, row) in self.iter_rows().enumerate() {
      let row_str = row.iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join("");

      writeln!(f, "{i}\t{row_str}")?;
    }

    std::fmt::Result::Ok(())
  }
}

impl<T: Default + Clone> Grid<T> {
  pub fn new(width: usize, height: usize) -> Self {
    Self {
      data: vec![Default::default(); width * height],
      width, height
    }
  }

  pub fn push_row(&mut self) {
    self.data.resize(self.data.len() + self.width, T::default());
  }

  pub fn resize(&mut self, height: usize, value: T) {
    self.data.resize(self.width * height, value);
  }
}

impl<T: Default> Grid<T> {
  pub fn pos_is_in_bounds(&self, (x, y): Coordinate) -> bool {
    x >= 0 && y >= 0 && x < self.width as isize && y < self.height as isize
  }

  fn pos_to_idx_unchecked(&self, (x, y): Coordinate) -> usize {
    y as usize * self.width + x as usize
  }

  pub fn pos_to_idx(&self, c: Coordinate) -> Option<usize> {
    if !self.pos_is_in_bounds(c) { None }
    else { Some(self.pos_to_idx_unchecked(c)) }
  }

  fn idx_to_pos_unchecked(&self, i: usize) -> Coordinate {
    ((i % self.width) as isize, (i / self.width) as isize)
  }

  pub fn idx_to_pos(&self, i: usize) -> Option<Coordinate> {
    if i >= self.data.len() { None }
    else {
      Some(self.idx_to_pos_unchecked(i))
    }
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

  pub fn pop_row(&mut self) {
    self.data.truncate(self.data.len() - self.width);
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

  pub fn iter_cols(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
    (0..self.width)
      .map(|col| self.data[col..].iter().step_by(self.width))
  }

  pub fn iter_coords(&self) -> impl Iterator<Item = (Coordinate, &T)> {
    self.iter().enumerate().map(|(i, v)| 
      (self.idx_to_pos_unchecked(i), v)
    )
  }

  pub fn iter_coords_mut(&mut self) -> impl Iterator<Item = (Coordinate, &mut T)> { 
    let width = self.width;
    
    self.iter_mut().enumerate().map(move |(i, v)| {
      (((i % width) as isize, (i / width) as isize), v)
    })
  }
}

impl<T: Default> Index<Coordinate> for Grid<T> {
  type Output = T;

  fn index(&self, c: Coordinate) -> &Self::Output {
    &self.data[self.pos_to_idx_unchecked(c)]
  }
}

impl<T: Default> Index<&Coordinate> for Grid<T> {
  type Output = T;

  fn index(&self, c: &Coordinate) -> &Self::Output {
    &self.data[self.pos_to_idx_unchecked(*c)]
  }
}

impl<T: Default> Index<&mut Coordinate> for Grid<T> {
  type Output = T;

  fn index(&self, c: &mut Coordinate) -> &Self::Output {
    &self.data[self.pos_to_idx_unchecked(*c)]
  }
}

impl<T: Default> IndexMut<Coordinate> for Grid<T> {
  fn index_mut(&mut self, c: Coordinate) -> &mut Self::Output {
    let idx = self.pos_to_idx_unchecked(c);
    &mut self.data[idx]
  }
}

impl<T: Default> IndexMut<&Coordinate> for Grid<T> {
  fn index_mut(&mut self, c: &Coordinate) -> &mut Self::Output {
    let idx = self.pos_to_idx_unchecked(*c);
    &mut self.data[idx]
  }
}

impl<T: Default> IndexMut<&mut Coordinate> for Grid<T> {
  fn index_mut(&mut self, c: &mut Coordinate) -> &mut Self::Output {
    let idx = self.pos_to_idx_unchecked(*c);
    &mut self.data[idx]
  }
}

impl From<&str> for Grid<char> {
  fn from(value: &str) -> Self {
    value.lines().map(|line| line.chars()).collect()
  }
}

impl<T: Clone + Default> From<&[&[T]]> for Grid<T> {
  fn from(value: &[&[T]]) -> Self {
    let width = value.first().map(|f| f.len()).expect("grid should not be empty");
    assert!(value.iter().all(|row| row.len() == width), "can't initialize grid: not all rows have a width equal to {width}");

    Self {
      data: value.concat(),
      width,
      height: value.len(),
    }
  }
}

impl<T: Clone + Default> From<&[Vec<T>]> for Grid<T> {
  fn from(value: &[Vec<T>]) -> Self {
    let width = value.first().map(|f| f.len()).expect("grid should not be empty");
    assert!(value.iter().all(|row| row.len() == width), "can't initialize grid: not all rows have a width equal to {width}");
    
    Self {
      data: value.concat(),
      width,
      height: value.len(),
    }
  }
}

impl<T: Clone + Default> From<&mut [Vec<T>]> for Grid<T> {
  fn from(value: &mut [Vec<T>]) -> Self {
    let width = value.first().map(|f| f.len()).expect("grid should not be empty");
    assert!(value.iter().all(|row| row.len() <= width), "can't initialize grid: not all rows have a width equal or less than {width}");

    value.iter_mut()
      .for_each(|row| row.resize(width, T::default()));

    Self {
      data: value.concat(),
      width,
      height: value.len(),
    }
  }
}

impl<T: Clone + Default> From<&Vec<Vec<T>>> for Grid<T> {
  fn from(value: &Vec<Vec<T>>) -> Self {
    Self::from(value.as_slice())
  }
}

impl<T: Clone + Default> From<&mut Vec<Vec<T>>> for Grid<T> {
  fn from(value: &mut Vec<Vec<T>>) -> Self {
    Self::from(value.as_slice())
  }
}

impl<T: Default> IntoIterator for Grid<T> {
  type Item = T;
  type IntoIter = std::vec::IntoIter<T>;

  fn into_iter(self) -> Self::IntoIter {
    self.data.into_iter()
  }
}

impl<T: Default, I> FromIterator<I> for Grid<T>
where 
  I: Iterator<Item = T> + Clone,
{
  fn from_iter<F: IntoIterator<Item = I>>(iter: F) -> Self {
    let mut iter = iter.into_iter();

    let first_row = iter.next().expect("grid should not be empty");
    let width = first_row.clone().count();
    let mut data = Vec::from_iter(first_row);

    for row in iter {
      assert!(row.clone().count() == width, "can't initialize grid: not all rows have a width equal to {width}");
      data.extend(row);
    }

    let len = data.len();
    Self {
      data: data,
      width,
      height: len / width
    }
  }
}

// impl<T: Default> FromIterator<Vec<T>> for Grid<T> {
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
//       data: data,
//       width,
//       height: len / width
//     }
//   }
// }