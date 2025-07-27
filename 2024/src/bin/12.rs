use std::{collections::{HashSet, VecDeque}};

use aoc2024::{coord_add, Coordinate, Grid, DIRECTIONS, DIRECTIONS_DIAG};

fn main() {
  let input = include_str!("12.txt");
  let grid = Grid::from(input);

  let mut res1 = 0;
  let mut res2 = 0;
  let mut visited = HashSet::new();
  for (pos, region) in grid.iter_coords() {
    if !visited.contains(&pos) {
      let (cells, edges, sides) = bfs(&grid, *region, pos, &mut visited);
      // area is simply the cells count
      let area = cells;

      // perimeter: we take the cells count and multiply by 4. we now have a summed perimeter as all cells wehre squares.
      // we count the inner edges, and remove them from the total perimeter. we have to multiply the edges count by 2,
      // as we have to remove each edge twice (the summed perimeter earlier had overlapping squares)
      let perimeter = cells*4 - edges*2;
      res1 += area * perimeter;
      res2 += area * sides;
    }
  }

  println!("{res1}");
  println!("{res2}");
}

fn bfs(
  grid: &Grid<char>,
  region: char,
  start: Coordinate,
  visited: &mut HashSet<Coordinate>
) -> (usize, usize, usize) {
  let mut queue = VecDeque::new();
  queue.push_back(start);
  visited.insert(start);

  let mut edges = HashSet::new();
  let mut cells = 1;
  let mut corners = 0;

  while !queue.is_empty() {
    let curr = queue.pop_front().unwrap();

    for dir in DIRECTIONS {
      let adj = coord_add(&curr, dir);

      if grid.pos_is_in_bounds(adj) && grid[adj] == region {
        if !visited.contains(&adj) {
          cells += 1;
          queue.push_back(adj);
          visited.insert(adj);
        }

        // save edge
        let mut edge1 = curr;
        let mut edge2 = adj;
        if edge1 > edge2 { std::mem::swap(&mut edge1, &mut edge2); }
        edges.insert((edge1, edge2));
      }
    }

    // check sides/corners if they are of the same region
    let mut neighbors = [false; 8];
    for (i, dir) in DIRECTIONS_DIAG.iter().enumerate() {
      let neighbor = coord_add(&curr, dir);
      if let Some(neighbor) = grid.get(neighbor) {
        neighbors[i] = *neighbor == region;
      }
    }

    // convex corners

    /*
      ?B
      BA
    */

    // up and left
    if !neighbors[1] && !neighbors[3] { corners += 1; }
    // up and right
    if !neighbors[1] && !neighbors[4] { corners += 1; }
    // down and left
    if !neighbors[6] && !neighbors[3] { corners += 1; }
    // down and right
    if !neighbors[6] && !neighbors[4] { corners += 1; }

    // concave corners

    /*
      BA
      AA
    */

    // up and left
    if neighbors[1] && neighbors[3] && !neighbors[0] { corners += 1; }
    // up and right
    if neighbors[1] && neighbors[4] && !neighbors[2] { corners += 1; }
    // down and left
    if neighbors[6] && neighbors[3] && !neighbors[5] { corners += 1; }
    // down and right
    if neighbors[6] && neighbors[4] && !neighbors[7] { corners += 1; }
  }

  (cells, edges.len(), corners)
}