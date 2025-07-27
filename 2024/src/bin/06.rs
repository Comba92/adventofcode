use std::collections::HashSet;

use aoc2024::{Coordinate, Grid};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
  Up, Right, Down, Left, None
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
  fn turn(&self) -> Self {
    match self {
      Self::Up    => Self::Right,
      Self::Right => Self::Down,
      Self::Down  => Self::Left,
      Self::Left  => Self::Up,
      Self::None  => Self::None,
    }
  }

  fn vector(&self) -> Coordinate {
    match self {
      Self::Up    => (0, -1),
      Self::Right => (1, 0),
      Self::Down  => (0, 1),
      Self::Left  => (-1, 0),
      Self::None  => (0, 0),
    }
  }
}

// 1231 -- wrong
// 1597 - even lower
// 1627 - too low
// 1860 -- wrong
// 1975 -- too high
// 2015 -- wrong
// 2092 -- wrong
// 3465 -- wrong
// 3536 -- wrong
// 3642 -- wrong 
// 3967 -- too high
// 3722 -- wrong

// 1951 ??

fn main() {
  let input = include_str!("06.txt");

  let mut grid = Grid::from(input);

  let (start_pos, dir) = grid
    .iter_coords()
    .filter(|(_, c)| **c != '.' && **c != '#')
    .next().unwrap();

  let mut curr_pos = start_pos;
  let mut direction = Direction::from(*dir);

  let mut visited = HashSet::with_capacity(grid.width * grid.height);
  visited.insert(curr_pos);

  loop {
    let next_pos = next_position(&curr_pos, &direction);

    if !grid.pos_is_in_bounds(next_pos) {
      break; 
    }

    if grid[next_pos] == '#' {
      direction = direction.turn();
    } else {
      curr_pos = next_pos;
      visited.insert(curr_pos);
    }
  }
  
  // we found way out;
  // now check for loops. iter each position, and check all the others, by putting a new visited.
  // if this new visited can connect to another visited with the same direction, then we made a loop.

  let mut obstacles = HashSet::new();
  for pos in &visited {
    let obstacle_pos = *pos;
    if !grid.pos_is_in_bounds(obstacle_pos) || grid[obstacle_pos] == '#' { continue; }

    grid[obstacle_pos] = '#';

    let mut looper_visited = HashSet::new();
    let mut curr_pos = start_pos;
    let mut direction = Direction::Up;

    loop {
      let next_pos = next_position(&curr_pos, &direction);

      if !grid.pos_is_in_bounds(next_pos) { break; }
  
      if grid[next_pos] == '#' {
        direction = direction.turn();
        looper_visited.remove(&(curr_pos, direction));
      } else {
        if looper_visited.contains(&(next_pos, direction)) {
          obstacles.insert(obstacle_pos);
          break;
        }
        curr_pos = next_pos;
        looper_visited.insert((next_pos, direction));
      }
    }

    grid[obstacle_pos] = '.';
  }

  let res1 = visited.len();
  let res2 = obstacles.len();

  println!("{res1}");
  println!("{res2}");
}

fn next_position(curr: &Coordinate, direction: &Direction) -> Coordinate {
  let dir = direction.vector();
  let x = curr.0 + dir.0;
  let y = curr.1 + dir.1;
  (x, y)
}
