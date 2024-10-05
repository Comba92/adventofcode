use std::collections::HashSet;

type Vector3D = (i32, i32, i32);
const DIRECTIONS: [Vector3D; 6] = [
  (-1, 0, 0), (1, 0, 0), (0, -1, 0), (0, 1, 0), (0, 0, -1), (0, 0, 1)
];

fn vec3D_add(a: Vector3D, b: Vector3D) -> Vector3D {
  (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

fn main() {
  let input = include_str!("18.txt");

  let points: HashSet<Vector3D> = input.lines()
  .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
  .map(|p: Vec<i32>| {
    let mut iter = p.iter();
    (*iter.next().unwrap(), *iter.next().unwrap(), *iter.next().unwrap())
  })
  .collect();

  let mut connected_sides = 0;

  let xs = points.iter().map(|p| p.0);
  let ys = points.iter().map(|p| p.1);
  let zs = points.iter().map(|p| p.2);

  let x_bounds = (xs.clone().min().unwrap(), xs.max().unwrap());
  let y_bounds = (ys.clone().min().unwrap(), ys.max().unwrap());
  let z_bounds = (zs.clone().min().unwrap(), zs.max().unwrap());

  // Naive O(n^2)
  // for first in 0..points.len() {
  //   for second in first..points.len() {
  //     let a = points[first];
  //     let b = points[second];

  //     if a.0 == b.0 && a.1 == b.1 {
  //       connected_sides += if a.2.abs_diff(b.2) == 1 { 2 } else { 0 };
  //     }
  //     else if a.1 == b.1 && a.2 == b.2 {
  //       connected_sides += if a.0.abs_diff(b.0) == 1 { 2 } else { 0 };
  //     }
  //     else if a.0 == b.0 && a.2 == b.2 {
  //       connected_sides += if a.1.abs_diff(b.1) == 1 { 2 } else { 0 };
  //     }
  //   }
  // }


  // O(n) with HashSet
  for a in &points {
    for direction in DIRECTIONS {
      let b = vec3D_add(*a, direction);
      if !points.contains(&b) { continue; }

      if a.0 == b.0 && a.1 == b.1 {
        connected_sides += if a.2.abs_diff(b.2) == 1 { 1 } else { 0 };
      }
      else if a.1 == b.1 && a.2 == b.2 {
        connected_sides += if a.0.abs_diff(b.0) == 1 { 1 } else { 0 };
      }
      else if a.0 == b.0 && a.2 == b.2 {
        connected_sides += if a.1.abs_diff(b.1) == 1 { 1 } else { 0 };
      }
    }
  }
  let result1 = (points.len() * 6) - connected_sides;

  // run a flood-fill outside, count how many sides you count
  let start = (x_bounds.0, y_bounds.1, z_bounds.0);
  


  println!("{result1}");
}