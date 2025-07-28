use std::collections::HashSet;

use aoc2024::Grid;

fn main() {
  let input = include_str!("25.txt");

  let schematics = input
    .split("\n\n")
    .map(|schematic| Grid::from(schematic))
    .collect::<Vec<_>>();
  
  let mut locks = HashSet::new();
  let mut keys = HashSet::new();

  for schematic in &schematics {
    let col_lens = schematic.iter_cols()
      .map(|col| col
        .filter(|c| **c == '#').count() - 1)
      .collect::<Vec<_>>();
    
    let is_key = schematic.iter_rows()
      .next().unwrap()
      .iter()
      .all(|c| *c == '.');

    if is_key {
      keys.insert(col_lens);
    } else {
      locks.insert(col_lens);
    }
  }

  let height = schematics[0].height - 1;
  let mut overlapping = 0;

  for key in &keys {
    for lock in &locks {
      let mut overlap = false;
      
      for (left, right) in key.iter().zip(lock.iter()) {
        if left + right >= height {
          overlap = true;
          break;
        }
      }

      if overlap { overlapping += 1; }
    }
  }

  let res1 = (keys.len() * locks.len()) - overlapping;
  println!("{res1}");
}