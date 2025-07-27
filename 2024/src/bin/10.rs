use std::collections::HashSet;

use aoc2024::{coord_add, Coordinate, Grid, DIRECTIONS};

fn main() {
  let input = include_str!("10.txt");

  let grid = Grid::from(input);

  let mut res1 = 0;
  let mut res2 = 0;
  for (pos, c) in grid.iter_coords() {
    if *c == '0' {
      let mut reached = HashSet::new();
      res2 += dfs(&grid, pos, *c, &mut HashSet::new(), &mut reached);
      res1 += reached.len();
    }
  }

  println!("{res1}");
  println!("{res2}");
}

fn dfs(
  grid: &Grid<char>,
  pos: Coordinate,
  curr: char,
  branches: &mut HashSet<Coordinate>,
  reached: &mut HashSet<Coordinate>
) -> usize {
  let mut paths = 0;

  for dir in DIRECTIONS {
    let next_pos = coord_add(&pos, dir);
    if !grid.pos_is_in_bounds(next_pos) { continue; }

    if grid[next_pos] as u8 == curr as u8 + 1 {
      if grid[next_pos] == '9' {
        reached.insert(next_pos);
        paths += 1;
      } else {
        paths += dfs(grid, next_pos, (curr as u8 + 1) as char, branches, reached);
      }
    }
  }

  paths
}