use aoc2024::DIRECTIONS_DIAG;

fn main() {
  let input = include_str!("04.txt");

  let matrix = input
    .lines()
    .map(|line| line.chars().collect::<Vec<_>>())
    .collect::<Vec<_>>();

  let mut res1 = 0;
  let mut res2 = 0;
  for (y, row) in matrix.iter().enumerate() {
    for (x, c) in row.iter().enumerate() {
      if *c == 'X' {
        let pos = (x as isize, y as isize);

        for direction in DIRECTIONS_DIAG {
          if dfs_single_direction(&matrix, "MAS", pos, *direction) {
            res1 += 1;
          }
        }
      } else if *c == 'A' && mas_cross(&matrix, x, y) {
        res2 += 1;
      }
    }
  }

  println!("{res1}");
  println!("{res2}");
}

fn dfs_single_direction(
  matrix: &Vec<Vec<char>>,
  target: &str,
  pos: (isize, isize),
  dir: (isize, isize)
) -> bool {
  if target.is_empty() { return true }
  
  let x = pos.0 + dir.0;
  let y = pos.1 + dir.1;
  if x < 0 || x >= matrix[0].len() as isize || y < 0 || y >= matrix.len() as isize {
    return false;
  }

  let c = matrix[y as usize][x as usize];

  if target.starts_with(c) {
    let next = (x, y);
    dfs_single_direction(matrix, &target[1..], next, dir)
  } else {
    false
  }
}

fn mas_cross(
  matrix: &Vec<Vec<char>>,
  x: usize, y: usize,
) -> bool {
  if x == 0 || x == matrix[0].len() - 1 
  || y == 0 || y == matrix.len() - 1 {
    return false;
  }

  let ul = matrix[y-1][x-1];
  let ur = matrix[y-1][x+1];
  let ll = matrix[y+1][x-1];
  let lr = matrix[y+1][x+1];

  const TARGET: &[char] = &['M', 'S'];

  // let are_s_or_m =
  //   TARGET.contains(&ul) &&
  //   TARGET.contains(&ur) &&
  //   TARGET.contains(&ll) && 
  //   TARGET.contains(&lr);

  // let is_cross = (ul == ur && ll == lr && ul != ll) || (ul == ll && ur == lr && ul != ur);
  // are_s_or_m && is_cross

  let mut a = [ul, lr];
  a.sort();
  let mut b = [ur, ll];
  b.sort();

  a == TARGET && b == TARGET
}

// fn dfs_all_directions(
//   matrix: &Vec<Vec<char>>, 
//   visited: &mut HashSet<(usize, usize)>,
//   target: &str, 
//   cx: isize, cy: isize,
//   count: &mut u32,
// ) {
//   if target.is_empty() {
//     println!("{visited:?}");
//     *count += 1;
//     return;
//   }

//   for direction in DIRECTIONS {
//     let x = (cx + direction.0).clamp(0,matrix[0].len() as isize - 1);
//     let y = (cy + direction.1).clamp(0, matrix.len() as isize - 1);
//     let coord = (x as usize, y as usize);

//     if visited.contains(&coord) {
//       continue;
//     }

//     let c = matrix[coord.1][coord.0];
//     if target.starts_with(c) {
//       visited.insert(coord);
//       dfs(matrix, visited, &target[1..], x, y, count);
//       visited.remove(&coord);
//     }
//   }
// }