use std::collections::HashSet;

use aoc2024::{Coordinate, Grid};

fn main() {
  let input = include_str!("12.txt");
  
  let grid = input
    .lines()
    .map(|line| line.chars())
    .collect::<Grid<_>>();

  let mut visited_total = HashSet::new();

  let mut res1 = 0;
  for (coord, cell) in grid.iter_coords() {
    if visited_total.contains(&coord) { continue; }

    let mut region_cells = HashSet::new();
    dfs(&grid, coord, *cell, &mut region_cells);

    let min_width = region_cells.iter().min_by_key(|e| e.0).map(|e| e.0).unwrap();
    let max_width = region_cells.iter().max_by_key(|e| e.0).map(|e| e.0).unwrap();

    let min_height = region_cells.iter().min_by_key(|e| e.1).map(|e| e.1).unwrap();
    let max_height = region_cells.iter().max_by_key(|e| e.1).map(|e| e.1).unwrap();

    let width = max_width - min_width + 1;
    let height = max_height - min_height + 1;
    let perimeter = (width + height) * 2;
    let area = width * height;

    println!("{perimeter} {area} = {}", perimeter * area);
    res1 += perimeter * area;

    visited_total.extend(region_cells);
  }

  println!("{res1}");
}

fn dfs(grid: &Grid<char>, curr: Coordinate, target: char, visited: &mut HashSet<Coordinate>) {
  const DIRECTIONS: &[Coordinate] = &[
    (-1, 0), (1, 0), (0, -1), (0, 1)
  ];

  visited.insert(curr);
  for dir in DIRECTIONS {
    let next = (curr.0 + dir.0, curr.1 + dir.1);

    let already_visited = *grid.get(next).unwrap_or(&'\0') != target || visited.contains(&next);
    if !already_visited { dfs(grid, next, target, visited); }
  }
}