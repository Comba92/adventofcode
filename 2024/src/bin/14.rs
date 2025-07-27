use std::{collections::HashSet, i32, usize};

use aoc2024::{coord_add, Coordinate, Grid, DIRECTIONS};

fn main() {
  let input = include_str!("14.txt");
  let mut robots = input.lines()
    .map(|line| line.split_once(' ').unwrap())
    .map(|(pos, vel)| {
      let pos_comp = pos[2..].split_once(',').unwrap();
      let vel_comp = vel[2..].split_once(',').unwrap();

      (
        (pos_comp.0.parse::<isize>().unwrap(), pos_comp.1.parse::<isize>().unwrap()),
        (vel_comp.0.parse::<isize>().unwrap(), vel_comp.1.parse::<isize>().unwrap()),
      )
    })
    .collect::<Vec<_>>();

  const WIDTH: isize  = 101;
  const HEIGHT: isize = 103;
  const HALF_WIDTH: isize = WIDTH / 2;
  const HALF_HEIGHT: isize = HEIGHT / 2;

  let mut grid = Grid::<usize>::new(WIDTH as usize, HEIGHT as usize);
  
  let mut max_component_size = 0;
  let mut min_seconds = i32::MAX;
  for i in 0..100 {    
    for (pos, vel) in &mut robots {
      if grid[*pos] > 0 { grid[*pos] -= 1; }

      *pos = coord_add(pos, vel);
      pos.0 = pos.0.rem_euclid(grid.width as isize);
      pos.1 = pos.1.rem_euclid(grid.height as isize); 

      grid[*pos] += 1;
    }

    let mut visited= HashSet::new();
    let mut components_count = 0;
    for (pos, val) in grid.iter_coords() {
      if *val > 0 {
        components_count = components_count.max(dfs(&grid, pos, &mut visited));
      }
    }

    if components_count > max_component_size {
      min_seconds = min_seconds.min(i);
      max_component_size = components_count;
    }
  }

  let mut quadrants = [0; 4];
  for (pos, robots) in grid.iter_coords() {
    if *robots == 0 { continue; }
    
    let quadrant = if pos.0 < HALF_WIDTH && pos.1 < HALF_HEIGHT {
      0
    } else if pos.0 > HALF_WIDTH && pos.1 < HALF_HEIGHT {
      1
    } else if pos.0 < HALF_WIDTH && pos.1 > HALF_HEIGHT {
      2
    } else if pos.0 > HALF_WIDTH && pos.1 > HALF_HEIGHT {
      3
    } else { continue; };

    quadrants[quadrant] += robots;
  }

  let res1: usize = quadrants.iter().filter(|n| **n != 0).product();
  println!("{res1}");
  println!("Max components: {} at seconds: {}", max_component_size, min_seconds);
}

fn dfs(grid: &Grid<usize>, curr: Coordinate, visited: &mut HashSet<Coordinate>) -> usize {
  let mut count = 0;
  for dir in DIRECTIONS {
    let pos = coord_add(&curr, dir);
    if !visited.contains(&pos) && grid.pos_is_in_bounds(pos) && grid[pos] > 0 {
      visited.insert(pos);
      count += 1 + dfs(grid, pos, visited);
    }
  }

  count
}

// 11
// 87