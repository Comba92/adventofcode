use aoc2025::{Coordinate, Grid};
use std::collections::HashMap;

fn main() {
    let input = include_str!("07.txt");

    let grid = Grid::from(input);
    let mut pos = grid.coord('S').unwrap();
    pos.1 += 1;

    let res1 = traverse1(&mut grid.clone(), pos);
    let res2 = traverse2(&grid, pos, &mut HashMap::new());
    println!("{res1} {res2}");
}

fn traverse1(grid: &mut Grid<char>, pos: Coordinate) -> usize {
    let curr = grid.get(&pos);
    match curr {
        None | Some('|') => 0,
        Some('.') => {
            grid[pos] = '|';
            traverse1(grid, (pos.0, pos.1 + 1))
        }
        Some('^') => {
            let left = traverse1(grid, (pos.0 - 1, pos.1));
            let right = traverse1(grid, (pos.0 + 1, pos.1));
            1 + left + right
        }
        _ => unreachable!("{curr:?}"),
    }
}

fn traverse2(grid: &Grid<char>, pos: Coordinate, memo: &mut HashMap<Coordinate, usize>) -> usize {
    let curr = grid.get(&pos);
    match curr {
        None => 1,
        Some('.') => traverse2(grid, (pos.0, pos.1 + 1), memo),
        Some('^') => match memo.get(&pos) {
            None => {
                let left = traverse2(grid, (pos.0 - 1, pos.1), memo);
                let right = traverse2(grid, (pos.0 + 1, pos.1), memo);
                let rec = left + right;

                memo.insert(pos, rec);
                rec
            }
            Some(val) => *val,
        },
        _ => unreachable!("{curr:?}"),
    }
}
