use std::{cmp::Reverse, collections::BinaryHeap};

type Vector = (i32, i32);
const DIRECTIONS: [Vector; 4] = [
  (-1, 0), (1, 0), (0, -1), (0, 1)
];


#[derive(Debug, Clone)]
enum Cell { Start, End, Loc(u32) }
impl Cell {
  fn heigth(&self) -> u32 {
    match self {
      Cell::Start => 1,
      Cell::End => 'z' as u32 + 1,
      Cell::Loc(c) => *c
    }
  }
}

fn main() {
  let input = include_str!("12.txt");

  let mut start  = (0, 0);
  let mut end = (0, 0);
  
  let grid: Vec<Vec<Cell>> = input.lines().enumerate()
  .map(|(y, line)| line.chars().enumerate().map(|(x, c)| {
    let pos = (x as i32, y as i32);
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
        Cell::Loc(c as u32 - 'a' as u32 + 1)
      }
    }}).collect())
    .collect();
  
}