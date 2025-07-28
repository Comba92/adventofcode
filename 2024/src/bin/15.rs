use aoc2024::{coord_add, Coordinate, Grid};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
  Up, Right, Down, Left, None
}

impl From<char> for Direction {
  fn from(value: char) -> Self {
    match value {
      '^' => Self::Up,
      '>' => Self::Right,
      'v' => Self::Down,
      '<' => Self::Left,
      _ => Self::None
    }
  }
}

impl Direction {
  fn to_vector(&self) -> Coordinate {
    match self {
      Self::Up    => (0, -1),
      Self::Right => (1, 0),
      Self::Down  => (0, 1),
      Self::Left  => (-1, 0),
      Self::None  => (0, 0),
    }
  }

  fn to_char(&self) -> char {
    match self {
      Self::Up => '^',
      Self::Right => '>',
      Self::Down => 'v',
      Self::Left => '<',
      Self::None => ' ',
    }
  }
}

// 1571274 too high

fn main() {
  let input = include_str!("15.txt");
  let (map, commands) = input.split_once("\n\n").unwrap();

  let mut grid1 = Grid::from(map);
  let mut grid2 = grid1.iter_rows()
    .map(|row| {
      row.iter().fold(Vec::new(), |mut acc, c| {
        match *c {
          '#' => acc.extend_from_slice(&['#', '#']),
          'O' => acc.extend_from_slice(&['[', ']']),
          '.' => acc.extend_from_slice(&['.', '.']),
          '@' => acc.extend_from_slice(&['@', '.']),
          _ => unreachable!()
        }

        acc
      }).into_iter()
    })
    .collect::<Grid<_>>();

  let commands = commands.chars()
    .filter_map(|c| match Direction::from(c) {
      Direction::None => None,
      _ => Some(Direction::from(c))
    })
    .collect::<Vec<_>>();

  let res1 = solve1(&mut grid1, &commands);
  let res2 = solve2(&mut grid2, &commands);

  println!("{res1}");
  println!("{res2}");
}


fn solve1(grid: &mut Grid<char>, commands: &[Direction]) -> isize {
  let mut robot = grid.iter_coords()
    .find(|(_, val)| **val == '@')
    .unwrap()
    .0;

  for cmd in commands {
    let dir = cmd.to_vector();
    let next_robot = coord_add(&robot, &dir);
    
    match grid[next_robot] {
      '.' => {
        grid[next_robot]  = '@';
        grid[robot] = '.';
        robot = next_robot;
      }
      'O' => {
        // find last rock
        // move robot to first rock if there's space
        // add rock after fast rock if there's space

        let mut last_rock = next_robot;
        while grid[last_rock] == 'O' {
          last_rock = coord_add(&last_rock, &dir);
        }
        
        if grid[last_rock] == '.' {
          grid[last_rock] = 'O';
          grid[next_robot] = '@';
          grid[robot] = '.';
          robot = next_robot;
        }
      }
      '#' => {}
      _ => unreachable!()
    }
  }

  grid.iter_coords()
    .filter(|(_, val)| **val == 'O')
    .map(|(pos, _)| 100 * pos.1 + pos.0)
    .sum::<isize>()
}

fn solve2(grid: &mut Grid<char>, commands: &[Direction]) -> isize {
  let mut robot = grid.iter_coords()
    .find(|(_, val)| **val == '@')
    .unwrap()
    .0;

  println!("{grid}");

  for cmd in commands {
    let dir = cmd.to_vector();
    let next_robot = coord_add(&robot, &dir);
    let c = grid[next_robot];

    match (c, cmd) {
      ('.', _) => {
        grid[next_robot]  = '@';
        grid[robot] = '.';
        robot = next_robot;
      }
      ('[' | ']', Direction::Up | Direction::Down) => {
        if dfs_check(grid, &next_robot, &dir) {
          dfs_move(grid, &next_robot, &dir);
          grid[next_robot] = '@';
          grid[robot] = '.';
          robot = next_robot;
        }
      }
      ('[' | ']', Direction::Left | Direction::Right) => {
        let mut last_rock = next_robot;

        // move until there are rocks
        while grid[last_rock] == c {
          // rocks are now long 2 cells
          last_rock = coord_add(&last_rock, &dir);
          last_rock = coord_add(&last_rock, &dir);
        }
        
        // if there is space
        if grid[last_rock] == '.' {
          // get max left and max right x coords 
          let left = next_robot.0.min(last_rock.0);
          let right = next_robot.0.max(last_rock.0);
          let y = next_robot.1;

          // switch every rock char in that range
          for x in left+1..right {
            let rock = &mut grid[(x, y)];
            let d = match rock {
              '[' => ']',
              ']' => '[',
              _ => c
            };
            *rock = d;
          }

          // switch the leftmost or rightmost
          let d = match grid[next_robot] {
            '[' => ']',
            ']' => '[',
            _ => c
          };
          grid[last_rock] = d;

          grid[next_robot] = '@';
          grid[robot] = '.';
          robot = next_robot;
        }
      }
      ('#', _) => {}
      _ => unreachable!()
    }
    println!("Move {}\n{grid}", cmd.to_char());
  }

  println!("{grid}");

  grid.iter_coords()
  .filter(|(_, val)| **val == '[')
  .map(|(pos, _)| 100 * pos.1 + pos.0)
  .sum::<isize>()
}

/*
  1.
  []
  []

  2.
  [][]
   []

  3.
  ..
  []
*/

fn dfs_check(grid: &Grid<char>, curr: &Coordinate, dir: &Coordinate) -> bool {
  let curr_c = grid[curr];
  let curr_l = (if curr_c == '[' { curr.0 } else { curr.0 - 1 }, curr.1);
  
  let next = coord_add(&curr_l, dir);
  let next_c = grid[next];
  let next_l = (if next_c == '[' { next.0 } else { next.0 - 1 }, next.1);
  let next_r = coord_add(&next_l, &(1, 0));

  match next_c {
    // case 1, one rock
    '[' => dfs_check(grid, &next_l, dir),
    // case 2, two rocks
    ']' => dfs_check(grid, &next_l, dir) 
      && dfs_check(grid, &next_r, dir),
    // case 3, both have to be '.'
    '.' => grid[next_r] == '.',
    '#' => false,
    _ => unreachable!()
  }
}

fn dfs_move(grid: &mut Grid<char>, curr: &Coordinate, dir: &Coordinate) {
  let curr_c = grid[curr];
  let curr_l = (if curr_c == '[' { curr.0 } else { curr.0 - 1 }, curr.1);
  let curr_r = coord_add(&curr_l, &(1, 0));
  
  let next = coord_add(&curr_l, dir);
  let next_c = grid[next];
  let next_l = (if next_c == '[' { next.0 } else { next.0 - 1 }, next.1);
  let next_r = coord_add(&next_l, &(1, 0));

  let move_rock_if_has_space = |grid: &mut Grid<char>| {
    if grid[next_r] == '.' {
      // case 3
      // we can move
      grid[next_l] = '[';
      grid[next_r] = ']';
      grid[curr_l] = '.';
      grid[curr_r] = '.';
    }
  };

  match next_c {
    '[' => {
      // case 1, one rock
      dfs_move(grid, &next_l, dir);
      move_rock_if_has_space(grid);
    }

    ']' => {
      // case 2, two rocks
      dfs_move(grid, &next_l, dir);
      dfs_move(grid, &(next_l.0 + 1, next_l.1), dir);
      move_rock_if_has_space(grid);
    }
    '.' => move_rock_if_has_space(grid),
    // we can't move, do nothing
    '#' => {}
    _ => unreachable!()
  }
}