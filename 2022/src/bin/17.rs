type Vector = (i32, i32);
const LEFT: Vector = (-1, 0);
const RIGHT: Vector = (1, 0);
const DOWN: Vector = (0, -1);

fn vec_add(a: Vector, b: Vector) -> Vector {
  (a.0 + b.0, a.1 + b.1)
}

struct RockKind {
  blocks: &'static [Vector],
  height: usize,
}

static BVERTICAL: [Vector; 4] = [
  (0, 0), (1, 0), (2, 0), (3, 0)
];
static VERTICAL: RockKind = RockKind { height: 1, blocks: &BVERTICAL };

static BCROSS: [Vector; 5] = [
  (1, 0), (0, -1), (1, -1), (2, -1), (1, -2)
];
static CROSS: RockKind = RockKind { height: 3, blocks: &BCROSS };

static BTRIANGLE: [Vector; 5] = [
  (2, 0), (2, -1), (0, -2), (1, -2), (2, -2)
];
static TRIANGLE: RockKind = RockKind { height: 3, blocks: &BTRIANGLE };

static BHORIZONTAL: [Vector; 4] = [
  (0, 0), (0, -1), (0, -2), (0, -3)
];
static HORIZONTAL: RockKind = RockKind { height: 4, blocks: &BHORIZONTAL };

static BSQUARE: [Vector; 4] = [
  (0, 0), (1, 0), (0, -1), (1, -1)
];
static SQUARE: RockKind = RockKind { height: 2, blocks: &BSQUARE };

static TO_SPAWN: [&RockKind; 5] = [&VERTICAL, &CROSS, &TRIANGLE, &HORIZONTAL, &SQUARE];

struct Rock {
  pos: Vector,
  blocks: Vec<Vector>,
  kind: &'static RockKind,
}
impl Rock {
  fn new(pos: Vector, kind: &'static RockKind) -> Self {
    let blocks = kind.blocks.iter().map(|&b| vec_add(pos, b)).collect();
    Rock {pos, blocks, kind}
  }

  fn next(&self, direction: Vector) -> Self {
    Rock::new(vec_add(self.pos, direction), &self.kind)
  }

  fn update(&mut self, direction: Vector) {
    *self = self.next(direction);
  }

  fn is_pos_valid(&self, direction: Vector, grid: &Vec<[bool; 7]>) -> bool {
    let next = self.next(direction);
    
    for block in &next.blocks {
      if block.0 < 0 || block.0 >= grid[0].len() as i32
      || block.1 < 0 || block.1 >= grid.len() as i32
      || grid[block.1 as usize][block.0 as usize] { return false }
    }

    true
  }

  fn set(&self, grid: &mut Vec<[bool; 7]>) {
    for block in &self.blocks {
      grid[block.1 as usize][block.0 as usize] = true;
    }
  }
}

fn draw_static(grid: &Vec<[bool; 7]>) {
  for y in 0..grid.len() {
    let y = grid.len() - y - 1;
    for x in 0..grid[0].len() {
      match grid[y][x] {
        true => print!("#"),
        false => print!(".")
      }
    }
    println!()
  }
  println!()
}

fn draw(grid: &Vec<[bool; 7]>, rock: &Rock) {
  for y in 0..grid.len() {
    let y = grid.len() - y - 1;

    for x in 0..grid[0].len() {
      let curr = (x as i32, y as i32);
      if rock.blocks.contains(&curr) {
        print!("#");
        continue
      }
      
      match grid[y][x] {
        true => print!("#"),
        false => print!(".")
      }
    }
    println!()
  }
  println!()
}

fn main() {
  let input = include_str!("17.txt");
  let pattern: Vec<Vector> = input.chars().map(|c| {
    match c {
      '<' => LEFT,
      '>' => RIGHT,
      _ => unreachable!()
    }
  }).collect();

  let mut grid: Vec<[bool; 7]> = Vec::new();

  for _ in 0..=100 * 100 { grid.push([false; 7]); }

  let mut spawn = TO_SPAWN.iter().enumerate().cycle();
  let mut wind = pattern.iter().enumerate().cycle();

  for _ in 0..2019 {
    let max_y = grid.iter().enumerate()
    .filter(|(_, row)| row.iter().any(|b| *b))
    .map(|(i, _)| i+1)
    .max().unwrap_or(0);

    let (spawn_idx, current_kind) = spawn.next().unwrap();
    let top = max_y + 3 + current_kind.height - 1;

    let current_pos = (2, top as i32);
    let mut current = Rock::new(current_pos, &current_kind);

    if top > grid.len() {
      let missing_rows = top - grid.len() + 1;
      for _ in 0..=missing_rows { grid.push([false; 7]); }
    }

    //draw(&grid, &current);

    loop {
      // draw(&grid, &current);
      //println!("{:?}\n\n", current.pos);

      let (wind_idx, &push) = wind.next().unwrap();
      if current.is_pos_valid(push, &grid) {
        current.update(push);
      }

      // draw(&grid, &current);
      //println!("{:?}\n\n", current.pos);
      
      if !current.is_pos_valid(DOWN, &grid) {
        current.set(&mut grid);
        break
      }

      current.update(DOWN);
    }
  }

  let result1 = grid.iter().enumerate()
  .filter(|(_, row)| row.iter().any(|b| *b))
  .map(|(i, _)| i+1)
  .max().unwrap();

  draw_static(&grid);

  // 3147 is too high!
  println!("{result1}");
}