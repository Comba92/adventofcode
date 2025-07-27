use aoc2024::{coord_add, Grid};

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

  let mut grid = Grid::<u8>::new(WIDTH as usize, HEIGHT as usize);
  
  for i in 0..100 {    
    for (pos, vel) in &mut robots {
      if grid[*pos] > 0 { grid[*pos] -= 1; }

      *pos = coord_add(pos, vel);
      pos.0 = pos.0.rem_euclid(grid.width as isize);
      pos.1 = pos.1.rem_euclid(grid.height as isize); 

      grid[*pos] += 1;
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

  let res1: usize = quadrants.iter()
    .filter(|n| **n != 0)
    .map(|n| *n as usize)
    .product();
  println!("{res1}");
}