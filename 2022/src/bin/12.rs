use std::collections::{BinaryHeap, HashMap};

mod utils;
use utils::{matrix::Matrix, vector::{Vector, DIRECTIONS}};

#[derive(Debug, Clone)]
enum Cell { Start, End, Loc(u32) }
impl Cell {
  fn heigth(&self) -> u32 {
    match self {
      Cell::Start => 1,
      Cell::End => ('z' as i32 - 'a' as i32 + 1) as u32,
      Cell::Loc(c) => *c + 1,
    }
  }
}

fn dijkstra(grid: &Matrix<Cell>, start: Vector, can_move: fn(u32, u32) -> bool, dist: &mut HashMap<Vector, usize>) {
  let mut queue = BinaryHeap::from([(0, start)]);
  dist.insert(start, 0);
  
  while let Some((cost, pos)) = queue.pop() {
    let curr = grid.get(&pos);

    // we skip this computation, as from this position, we already found a better path
    if cost > *dist.get(&pos).unwrap() { continue; }

    for direction in DIRECTIONS {
      let next_pos = (pos.0 + direction.0, pos.1 + direction.1);
      if !grid.is_in_bounds(&next_pos) { continue; }

      let next = grid.get(&next_pos);
      if !can_move(curr.heigth(), next.heigth()) { continue; }
      
      if cost + 1 < *dist.get(&next_pos).unwrap() {
        queue.push((cost + 1, next_pos));
        dist.insert(next_pos, cost + 1);
      }
    }
  }
}

fn main() {
  let input = include_str!("12.txt");

  let mut start  = (0, 0);
  let mut end = (0, 0);
  
  let mut dist = HashMap::new();
  
  let data: Vec<Vec<Cell>> = input.lines().enumerate()
  .map(|(y, line)| line.chars().enumerate().map(|(x, c)| {
    let pos = (x as i32, y as i32);
    dist.insert(pos, usize::MAX);
    match c {
      'S' => {
        start = pos;
        Cell::Start
      },
      'E' => {
        end = pos;
        Cell::End
      }
      _ => {
        Cell::Loc(c as u32 - 'a' as u32)
      }
    }}).collect())
    .collect();
  
  let grid = Matrix::new(data);
  
  dijkstra(&grid, start, |a, b| a + 1 >= b, &mut dist);
  let result1 = dist.get(&end).unwrap();
  print!("{result1} ");

  dist.iter_mut().for_each(|(_, d)| *d = usize::MAX);
  dijkstra(&grid, end, |a, b| a - 1 <= b, &mut dist);

  let result2 = dist.iter()
  .filter(|(pos, _)| grid.get(pos).heigth() == 1)
  .map(|(_, cost)| cost)
  .min().unwrap();

  println!("{result2}");
}