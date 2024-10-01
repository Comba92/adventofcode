use std::collections::HashSet;

type Vector = (i32, i32);
const LEFT: Vector = (-1, 0);
const RIGHT: Vector = (1, 0);
const UP: Vector = (0, -1);
const DOWN: Vector = (0, 1);

fn vec_add(vec1: Vector, vec2: Vector) -> Vector {
  (vec1.0 + vec2.0, vec1.1 + vec2.1)
}

const MIN_DIST: i32 = 2;
const KNOTS1: usize = 2;
const KNOTS2: usize = 10;

#[derive(Debug, Default)]
struct Knot {
  pos: Vector,
  prev: Vector,
}

fn solve(input: &str, knots_count: usize) -> usize {
  let mut visited: HashSet<Vector> = HashSet::new();
  let mut knots: Vec<Knot> = (0..knots_count).map(|_| Knot::default()).collect();

  for (i, line) in input.lines().enumerate() {
      let mut motions = line.split_whitespace();
      let direction = motions.next().unwrap();
      let steps: usize = motions.next().unwrap().parse().unwrap();

      for _ in 0..steps {
        knots[0].prev = knots[0].pos;
        let head = &mut knots[0].pos;
        match direction {
          "U" => {
            *head = vec_add(*head, UP);
          },
          "D" => {
            *head = vec_add(*head, DOWN);
          },
          "L" => {
            *head = vec_add(*head, LEFT);
          },
          "R" => {
            *head = vec_add(*head, RIGHT);
          },
          _ => unreachable!()
        }

        for i in 0..knots.len()-1 {
          let first = &knots[i].pos;
          let second = &knots[i+1].pos;
          let distance = (first.0 - second.0).pow(2) + (first.1 - second.1).pow(2);

          knots[i+1].prev = knots[i+1].pos.clone();
          
          if distance >= MIN_DIST.pow(2) {
            knots[i+1].pos = knots[i].prev.clone();
          }
        }

        visited.insert(knots.last().unwrap().pos);
      }

      println!("{i} -- {:?}", knots);
      println!("");
    }

  visited.len()
}

fn main() {
  let input = include_str!("09.txt");

  let result1 = solve(input, KNOTS1);
  let result2 = solve(input, KNOTS2);

  println!("{result1} {result2}");
}