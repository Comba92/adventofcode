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
  fn vector(&self) -> Coordinate {
    match self {
      Self::Up    => (0, -1),
      Self::Right => (1, 0),
      Self::Down  => (0, 1),
      Self::Left  => (-1, 0),
      Self::None  => (0, 0),
    }
  }
}

fn main() {
  let input = include_str!("15.txt");
  let (map, commands) = input.split_once("\n\n").unwrap();

  let mut grid = Grid::from(map);
  let mut grid_big = grid.iter_rows()
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
      _ => Some(Direction::from(c).vector())
    })
    .collect::<Vec<_>>();

  let mut robot = grid.iter_coords()
    .find(|(_, val)| **val == '@')
    .unwrap()
    .0;

  for dir in commands {
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

        let mut rock = next_robot;
        while grid[rock] == 'O' {
          rock = coord_add(&rock, &dir);
        }
        
        if grid[rock] == '.' {
          grid[rock] = 'O';
          grid[next_robot] = '@';
          grid[robot] = '.';
          robot = next_robot;
        }
      }
      '#' => {}
      _ => unreachable!()
    }
  }

  println!("{grid}");

  let res1 = grid.iter_coords()
    .filter(|(_, val)| **val == 'O')
    .map(|(pos, _)| 100 * pos.1 + pos.0)
    .sum::<isize>();

  println!("{res1}");
}