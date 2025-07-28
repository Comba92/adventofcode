use std::collections::HashSet;

use aoc2024::{coord_add, Coordinate, Grid, DIRECTIONS};

fn main() {
  let input = include_str!("16.txt");

  let grid = Grid::from(input);
  let start = grid.iter_coords()
    .find(|(_, c)| **c == 'S')
    .unwrap()
    .0;

  // let end = grid.iter_coords()
  //   .find(|(_, c)| **c == 'E')
  //   .unwrap()
  //   .0;

  let res1 = dfs(&grid, &start, &mut HashSet::new());
  println!("{res1}");
}

fn dfs(grid: &Grid<char>, pos: &Coordinate, visited: &mut HashSet<Coordinate>) -> usize {
  visited.insert(*pos);
  let mut min_score = usize::MAX;
  
  for dir in DIRECTIONS {
    let next = coord_add(pos, dir);

    let score = if grid[next] == '.' && !visited.contains(&next) {
      let rec = dfs(grid, &next, &mut visited.clone());
      if rec != usize::MAX {
        rec + 1
      } else {
        rec
      }
    } else if grid[next] == 'E' {
      1
    } else {
      usize::MAX
    };

    min_score = min_score.min(score);
  }

  min_score
}