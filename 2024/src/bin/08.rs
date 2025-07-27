use aoc2024::{coord_add, coord_sub, Coordinate, Grid};
use std::collections::{HashMap, HashSet};

fn main() {
  let input = include_str!("08.txt");
  let grid = Grid::from(input);

  let mut antennas: HashMap<char, HashSet<Coordinate>> = HashMap::new();

  for (pos, cell) in grid.iter_coords()
    .filter(|(_, c)| **c != '.')
  {
    antennas.entry(*cell).or_default().insert(pos);
  }

  let mut antinodes1 = HashSet::new();
  let mut antinodes2 = HashSet::new();
  for antenna in antennas.values() {
    let frequencies = Vec::from_iter(antenna.iter());

    for i in 0..frequencies.len()-1 {
      for j in i+1..frequencies.len() {
        let mut pair = [frequencies[i], frequencies[j]];
        
        // pair[0] is always the biggest
        if pair[0] > pair[1] { pair.swap(0, 1); }
  
        antinodes2.insert(pair[0].clone());
        antinodes2.insert(pair[1].clone());

        let dist = (pair[0].0 - pair[1].0, pair[0].1 - pair[1].1);

        // here add, as pair[0] is the biggest (and lower)
        let mut antinode1 = coord_add(pair[0], &dist);
        if grid.pos_is_in_bounds(antinode1) {
          antinodes1.insert(antinode1);
        }
        while grid.pos_is_in_bounds(antinode1) {
          antinodes2.insert(antinode1);
          antinode1 = coord_add(&antinode1, &dist);
        }
        
        
        // here add, as pair[1] is the smaller (and higher)
        let mut antinode2 = coord_sub(pair[1], &dist);
        if grid.pos_is_in_bounds(antinode2) {
          antinodes1.insert(antinode2);
        }
        while grid.pos_is_in_bounds(antinode2) {
          antinodes2.insert(antinode2);
          antinode2 = coord_sub(&antinode2, &dist);
        }
      }
    }
  }

  let res1 = antinodes1.len();
  let res2 = antinodes2.len();
  println!("{res1}");
  println!("{res2}");
}
