use std::{array::IntoIter, borrow::Borrow, collections::HashSet, fmt::Write, i128, ops::{Index, IndexMut}, str};

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

#[derive(PartialEq, Eq, Default, Clone)]
pub struct Grid<T: Default> {
  pub data: Vec<T>,
  pub width: usize,
  pub height: usize,
}

impl<T: std::fmt::Debug + Default> std::fmt::Debug for Grid<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    // let mut res = f.debug_list();
    
    // for item in self.data.iter().as_slice().chunks(self.width) {
    //   res.entry(&item);
    // }
    f.write_str("[\n")?;

    for item in self.data.iter().as_slice().chunks(self.width) {
      f.write_fmt(format_args!("  {:?}", item))?;
      f.write_str(",\n")?;
    }

    f.write_char(']')
  }
}

impl<T: Default + Clone> Grid<T> {
  pub fn new(width: usize, height: usize) -> Self {
    Self {
      data: vec![T::default(); width * height],
      width, height
    }
  }

  pub fn new_with<V: IntoIterator<Item = T>>(width: usize, height: usize, iter: impl IntoIterator<Item = V>) -> Self {
     let mut data = Vec::new();

      for (i, row) in iter.into_iter().enumerate() {
        data.extend(row);
        // be sure each row is the same size
        data.resize_with((i+1) * width, || T::default());
      }

      Self {
        data,
        width,
        height
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
  pub fn pos_is_in_bounds(&self, (x, y): &Coordinate) -> bool {
    let x = *x;
    let y = *y;
    x >= 0 && y >= 0 && x < self.width as isize && y < self.height as isize
  }

  fn pos_to_idx_unchecked(&self, (x, y): &Coordinate) -> usize {
    *y as usize * self.width + *x as usize
  }

  pub fn pos_to_idx(&self, c: &Coordinate) -> Option<usize> {
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

  pub fn get(&self, pos: &Coordinate) -> Option<&T> {
    if self.pos_is_in_bounds(pos) {
      Some(&self[pos])
    } else {
      None
    }
  }

  pub fn get_mut(&mut self, pos: &Coordinate) -> Option<&mut T> {
    if self.pos_is_in_bounds(pos) {
      Some(&mut self[pos])
    } else {
      None
    }
  }

  pub fn get_row(&mut self, row: usize) -> Option<&mut [T]> {
    if self.pos_is_in_bounds(&(0, row as isize)) {
      let start = self.width * row;
      Some(&mut self.data[start..start+self.width])
    } else {
      None
    }
  }

  pub fn set(&mut self, pos: &Coordinate, val: T) -> bool {
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

  pub fn iter_rows_enumerated(&self) -> impl Iterator<Item = (Coordinate, &T)> {
    self.iter().enumerate().map(|(i, v)| 
      (self.idx_to_pos_unchecked(i), v)
    )
  }

  pub fn iter_rows_enumerated_mut(&mut self) -> impl Iterator<Item = (Coordinate, &mut T)> { 
    let width = self.width;
    
    self.iter_mut().enumerate().map(move |(i, v)| {
      (((i % width) as isize, (i / width) as isize), v)
    })
  }

  pub fn iter_coords(&self) -> impl Iterator<Item = Coordinate> {
    self.iter().enumerate().map(|(i, _)| self.idx_to_pos_unchecked(i))
  }
}

impl<T: Default, C: Borrow<Coordinate>> Index<C> for Grid<T> {
  type Output = T;

  fn index(&self, idx: C) -> &Self::Output {
    &self.data[self.pos_to_idx_unchecked(idx.borrow())]
  }
}

impl<T: Default, C: Borrow<Coordinate>> IndexMut<C> for Grid<T> {
  fn index_mut(&mut self, pos: C) -> &mut Self::Output {
    let idx = self.pos_to_idx_unchecked(pos.borrow());
    &mut self.data[idx]
  }
}

impl<T: Default> IntoIterator for Grid<T> {
  type Item = T;
  type IntoIter = std::vec::IntoIter<Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    self.data.into_iter()
  }
}

// impl<T: Default + Clone, V: AsRef<[T]>> FromIterator<V> for Grid<T> {
//   fn from_iter<I: IntoIterator<Item = V>>(iter: I) -> Self {
//     // we sadly have to do this to get the max width
//     let buf: Vec<_> = iter.into_iter().collect();

//     if let Some(width) = buf.iter().map(|x| x.as_ref().len()).max() {
//       let height = buf.len();
//       let mut data = Vec::new();

//       for (i, row) in buf.into_iter().enumerate() {
//         data.extend_from_slice(row.as_ref());

//         // be sure each row is the same size
//         data.resize_with((i+1) * width, || T::default());
//       }

//       Self {
//         data,
//         width,
//         height
//       }
//     } else {
//       Self::new(0, 0)
//     }
//   }
// }

impl<T: Default + Clone, V: Iterator<Item = T> + Clone> FromIterator<V> for Grid<T> {
  fn from_iter<I: IntoIterator<Item = V>>(iter: I) -> Self {
    // we sadly have to do this to get the max width
    let buf: Vec<_> = iter.into_iter().collect();

    if let Some(width) = buf.iter().map(|x| x.clone().count()).max() {
      let height = buf.len();
      Self::new_with(width, height, buf)
    } else {
      Self::new(0, 0)
    }
  }
}

impl<'a> From<str::Lines<'a>> for Grid<char> {
  fn from(value: str::Lines) -> Self {
    value.map(|line| line.chars()).collect()
  }
}

impl From<&str> for Grid<char> {
  fn from(value: &str) -> Self {
    Grid::from(value.lines())
  }
}

fn main() {
  let input = include_str!("04.txt");

  let mut grid = Grid::from(input);

  let mut res1 = 0;
  for (pos, _) in grid.iter_rows_enumerated().filter(|(_, c)| **c == '@') {
    let mut neighbors = 0;
    
    for dir in DIRECTIONS_DIAG {
      let neighbor = (pos.0 + dir.0, pos.1 + dir.1);
      if grid.get(&neighbor).filter(|x| **x == '@').is_some() {
        neighbors += 1;
      }
    }

    if neighbors < 4 {
      res1 += 1;
    }
  }

  let mut res2 = 0;
  loop {
    let mut removed = HashSet::new();

    for (pos, _) in grid.iter_rows_enumerated().filter(|(_, c)| **c == '@') {
      let mut neighbors = 0;
      
      for dir in DIRECTIONS_DIAG {
        let neighbor = (pos.0 + dir.0, pos.1 + dir.1);
        if grid.get(&neighbor).filter(|x| **x == '@').is_some() {
          neighbors += 1;
        }
      }

      if neighbors < 4 {
        removed.insert(pos);
      }
    }

    let removed_count = removed.len();

    for pos in removed {
      grid.set(&pos, '.');
    }

    res2 += removed_count;
    if removed_count == 0 { break; }
  }

  println!("{res1} {res2}");
}