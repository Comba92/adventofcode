use aoc2024::{coord_add, coord_sub, Coordinate, Grid};
use std::collections::{HashMap, HashSet};

fn main() {
  let input = include_str!("08.txt");
  let mut grid = Grid::from(input);

  let mut antennas: HashMap<char, HashSet<Coordinate>> = HashMap::new();

  for (pos, cell) in grid.iter_coords() {
    antennas.entry(*cell).or_default().insert(pos);
  }

  for antenna in antennas.values() {
    for mut pair in antenna.iter().zip(antenna.iter().skip(1)) {
      if pair.0 >= pair.1 {
        std::mem::swap(&mut pair.0, &mut pair.1);
      }

      let dist = distance(&pair.0, &pair.1);
      let antinode1 = coord_sub(pair.0, &dist);
      match grid.get(antinode1) {
        Some(c) => if *c == '.' {
          grid.set(antinode1, '#');
        }
        None => { grid.set(antinode1, '#'); }
      }

      let antinode2 = coord_add(pair.1, &dist);
      match grid.get(antinode2) {
        Some(c) => if *c == '.' {
          grid.set(antinode2, '#');
        }
        None => { grid.set(antinode2, '#'); }
      }
    }
  }

  let res1 = grid.iter()
    .filter(|c| **c == '#')
    .count();

  println!("{grid}");
  println!("{res1}");
}

fn distance(a: &Coordinate, b: &Coordinate) -> (isize, isize) {
  (a.0.abs_diff(b.0) as isize, a.1.abs_diff(b.1) as isize)
}