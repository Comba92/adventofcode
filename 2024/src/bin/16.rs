use std::{cmp, collections::{BinaryHeap, HashMap, HashSet, VecDeque}, path, usize};

use aoc2024::{coord_add, Coordinate, Direction, Grid, DIRECTIONS};

fn main() {
  let input = include_str!("16.txt");

  let mut grid = Grid::from(input);
  let start = grid.iter_coords()
    .find(|(_, c)| **c == 'S')
    .unwrap()
    .0;

  let end = grid.iter_coords()
    .find(|(_, c)| **c == 'E')
    .unwrap()
    .0;


  let (res1, paths) = dijkstra(&grid, start, end);
  // let paths = floyd_warshall(&grid);

  // let mut minpaths = HashSet::new();
  // let mut curr = end;
  // while start != curr {
  //   curr = paths[&(start, curr)];
  //   minpaths.insert(curr);
  // }

  // for coord in minpaths {
  //   grid.set(coord, 'O');
  // }

  let mut visited = HashSet::new();
  let mut queue = VecDeque::new();

  visited.insert(end);
  queue.push_back(end);

  while let Some(coord) = queue.pop_front() {
    if coord == start { break; }
    for node in &paths[&coord] {
      if !visited.contains(node) {
        visited.insert(*node);
        queue.push_front(*node);
      }
    }
  }

  for coord in &visited {
    grid.set(*coord, 'O');
  }

  println!("{grid}");

  println!("Shortest path: {} - Actual nodes computed by dijkstra: {}", visited.len(), paths.len());

  println!("{res1:?}");
  // println!("{res2:?}");
}

#[derive(PartialEq, Eq)]
struct GridNode {
  coord: Coordinate,
  cost: usize,
  dir: Direction,
}
impl GridNode {
  pub fn new(coord: Coordinate, cost: usize, dir: Direction) -> Self {
    GridNode { coord, cost, dir }
  }
}

impl Ord for GridNode {
  fn cmp(&self, other: &Self) -> cmp::Ordering {
    // we need a minheap
    other.cost.cmp(&self.cost)
  }
}

impl PartialOrd for GridNode {
  fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
    Some(self.cmp(other))
  }
}

type GridPath = HashMap<Coordinate, HashSet<Coordinate>>;
fn dijkstra(grid: &Grid<char>, start: Coordinate, end: Coordinate) -> (usize, GridPath) {
  let mut distances = HashMap::new();
  let mut heap = BinaryHeap::new();
  let mut path = HashMap::new();

  distances.insert(start, 0);
  heap.push(GridNode::new(start, 0, Direction::Right));
  path.insert(start, HashSet::from([start]));

  for (coord, _) in grid.iter_coords()
    .filter(|(_, v)| **v != '#')
  {
    distances.insert(coord, usize::MAX);
  }

  let mut min_score = usize::MAX;
  while let Some(GridNode {coord, cost, dir, .. }) = heap.pop() {
    if coord == end {
      min_score = cost;
      continue;
    };
    if cost >= distances[&coord] { continue; }

    for next_dir in DIRECTIONS {
      let next_coord = coord_add(&coord, &next_dir);
      if grid[next_coord] == '#' { continue; }

      let next_dir = Direction::from(*next_dir);
      let score = if dir == next_dir { 1 } else { 1001 };
      let next_cost = cost + score;

      if next_cost < distances[&next_coord] {
        distances.insert(next_coord, next_cost);
        heap.push(GridNode::new(next_coord, next_cost, next_dir));
        path.entry(next_coord).or_default().insert(coord);
      }
    }
  }

  (min_score, path)
}

fn floyd_warshall(grid: &Grid<char>) -> HashMap<(Coordinate, Coordinate), Coordinate> {
  let mut distances = HashMap::new();
  let mut path = HashMap::new();
  
  let nodes = grid.iter_coords()
      .filter(|(_, v)| **v != '#')
      .map(|(c,_)| c)
      .collect::<Vec<_>>();

  // populate distances with bfs?
  for u in &nodes {
    for v in &nodes {
      if u == v { 
        distances.insert((v, v), 0);
      } else {
        distances.insert((u, v), usize::MAX); // this is wrong
        // should insert the weight
      }
      path.insert((*u, *v), *u);
    }
  }

  for k in &nodes {
    for i in &nodes {
      for j in &nodes {
        let ij = distances[&(i, j)];
        let ik = distances[&(i, k)];
        let kj = distances[&(k, j)];

        if ik == usize::MAX || kj == usize::MAX {
          continue;
        }

        if ij > ik + kj {
          distances.insert((i, j), ik + kj);
          path.insert((*i, *j), path[&(*k, *j)]);
        }
      }
    }
  }

  path
}