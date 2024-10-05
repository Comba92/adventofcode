type Vector = (i32, i32);
const DOWN: Vector = (0, 1);
const DOWNLEFT: Vector = (-1, 1);
const DOWNRIGHT: Vector = (1, 1);

#[derive(PartialEq, Eq, Clone)]
enum Entity {
  Air, Rock, Spawn, Sand
}

enum SandState {
  Falling(Vector), Void
}

fn draw(grid: &Vec<Vec<Entity>>) {
  for row in grid {
    for col in row {
      let c = match col {
        Entity::Air => '.',
        Entity::Rock => '#',
        Entity::Sand => 'o',
        Entity::Spawn => '+',
      };

      print!("{c}");
    }
    println!()
  }
  println!()
}

fn sand_next_pos(a: Vector, b: Vector, grid: &Vec<Vec<Entity>>) -> Option<SandState> {
  let x = a.0 + b.0;
  let y = a.1 + b.1;

  if x < 0 || x >= grid[0].len() as i32 || y < 0 || y >= grid.len() as i32 {
    Some(SandState::Void)
  }
  else if grid[y as usize][x as usize] != Entity::Air { None }
  else { Some(SandState::Falling((x, y))) }
}

fn solve(grid: &mut Vec<Vec<Entity>>, spawn_pos: (usize, usize), void: bool) -> usize {
  let spawn = (spawn_pos.0 as i32, spawn_pos.1 as i32);

  'game: loop {
      let mut sand = spawn;
      loop {
        let next = sand_next_pos(sand, DOWN, &grid)
          .or_else(|| sand_next_pos(sand, DOWNLEFT, &grid))
          .or_else(|| sand_next_pos(sand, DOWNRIGHT, &grid));

        // at rest
        if next.is_none() { 
          grid[sand.1 as usize][sand.0 as usize] = Entity::Sand;
          
          if sand == spawn {
            break 'game
          }
          else { break }
        }
        if let Some(state) = next {
          match state {
            SandState::Falling(pos) => sand = pos,
            SandState::Void => if void { break 'game },
          }
        }
      }
      // draw(&grid);
    }

  grid.concat().iter()
  .filter(|&e| *e == Entity::Sand)
  .count()
}

const SCALING_FACTOR: usize = 10;

fn main() {
  let input = include_str!("14.txt");

  let mut rocks = Vec::new();
  rocks.push((500, 0));

  for line in input.lines() {
    let path: Vec<(usize, usize)> = line.split("->").map(|r| {
      let point: Vec<usize> = r.split(',').map(|s| s.trim().parse().unwrap()).collect();
      (point[0], point[1])
    }).collect();

    // build lines
    for path_line in path.as_slice().windows(2) {
      let mut path_line = path_line.to_owned();
      path_line.sort();

      let first = path_line[0];
      let second = path_line[1];

      if first.0 == second.0 {
        for y in first.1 ..= second.1 {
          rocks.push((first.0, y));
        }
      } else {
        for x in first.0 ..= second.0 {
          rocks.push((x, first.1));
        }
      }
    }
  }
  
  let xs = rocks.iter().map(|p| p.0);
  let ys = rocks.iter().map(|p| p.1);
  let x_range = (xs.clone().min().unwrap(), xs.max().unwrap());
  let y_range = (ys.clone().min().unwrap(), ys.max().unwrap());
  let original_width = x_range.1 - x_range.0 + 1;
  let width =  original_width * SCALING_FACTOR;
  let height = y_range.1 - y_range.0 + 1;
  
  let mut grid: Vec<Vec<Entity>> = (0..height).map(|_| (0..width).map(|_| Entity::Air).collect()).collect();
  rocks.iter().for_each(|r| {
    let pos = ((r.0 - x_range.0) + width/2 - original_width/2, r.1 - y_range.0);
    grid[pos.1][pos.0] = Entity::Rock;
  });
  
  let spawn_pos = ((500 - x_range.0) as usize + width/2 - original_width/2, 0);
  let spawn = &mut grid[spawn_pos.1][spawn_pos.0];
  *spawn = Entity::Spawn;

  let result1 = solve(&mut grid.clone(), spawn_pos, true);

  grid.push((0..width).map(|_| Entity::Air).collect());
  grid.push((0..width).map(|_| Entity::Rock).collect());
  let result2 = solve(&mut grid, spawn_pos, false);

  println!("{result1} {result2}")
}
  
  