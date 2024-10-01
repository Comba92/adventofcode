use std::collections::HashSet;

const DIRECTIONS: [(i32, i32); 4] = [
  (-1, 0), (1, 0), (0, -1), (0, 1)
];

fn dfs1(
  grid: &Vec<Vec<u32>>, 
  first: u32, current: (i32, i32), 
  direction: (i32, i32),
  visited: &mut HashSet<(i32, i32)>
) -> bool 
{
  visited.insert(current);

  let x = current.0 + direction.0;
  let y = current.1 + direction.1;

  if x < 0 || x >= (grid[0].len()) as i32
  || y < 0 || y >= (grid.len()) as i32 {
    return true;
  }
  
  let to_visit = (x, y);
  if visited.contains(&to_visit)
  || first <= grid[y as usize][x as usize] { 
    return false;
  }

  return dfs1(grid, first, to_visit, direction, visited);
}

fn dfs2(
  grid: &Vec<Vec<u32>>, 
  first: u32, current: (i32, i32), 
  direction: (i32, i32),
  visited: &mut HashSet<(i32, i32)>
) -> u32
{
  visited.insert(current);

  let x = current.0 + direction.0;
  let y = current.1 + direction.1;

  if x < 0 || x >= (grid[0].len()) as i32
  || y < 0 || y >= (grid.len()) as i32 {
    // No tree visible after this
    return 0;
  }
  
  let to_visit = (x, y);
  if visited.contains(&to_visit)
  || first <= grid[y as usize][x as usize] { 
    // Tree visible after this!
    return 1;
  }

  let rec = dfs2(grid, first, to_visit, direction, visited);
  return rec + 1;
}

fn main() {
  let input = include_str!("08.txt");
  let mut grid: Vec<Vec<u32>> = Vec::new();

  for line in input.lines() {
    let nums = line.chars().map(|c| c.to_digit(10).unwrap());
    grid.push(Vec::from_iter(nums));
  }

  let mut result1: u32 = 0;
  let mut result2 = 0;
  for row in 0..grid.len() {
    for col in 0..grid[0].len() {
      let current = (col as i32, row as i32);
      let mut visible = false;
      let mut score = 1;

      for direction in DIRECTIONS {
        if dfs1(&grid, grid[row][col], current, direction, &mut HashSet::new()) {
          visible = true;
        }

        score *= dfs2(&grid, grid[row][col], current, direction, &mut HashSet::new());
      }

      if visible { result1 += 1; }
      result2 = result2.max(score);
    }
  }

  println!("{result1} {result2}");
}