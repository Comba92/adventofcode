use aoc2025::{DIRECTIONS_DIAG, Grid};
use std::collections::HashSet;

fn main() {
    let input = include_str!("04.txt");

    let mut grid = Grid::from(input);

    let mut res1 = 0;
    for (pos, _) in grid.iter_rows_enumerated().filter(|(_, c)| **c == '@') {
        let mut neighbors = 0;

        for dir in DIRECTIONS_DIAG {
            let neighbor = (pos.0 + dir.0, pos.1 + dir.1);
            if grid.get(&neighbor).filter(|x| **x == '@').is_some() {
                neighbors += 1;
            }
        }

        if neighbors < 4 {
            res1 += 1;
        }
    }

    let mut res2 = 0;
    loop {
        let mut removed = HashSet::new();

        for (pos, _) in grid.iter_rows_enumerated().filter(|(_, c)| **c == '@') {
            let mut neighbors = 0;

            for dir in DIRECTIONS_DIAG {
                let neighbor = (pos.0 + dir.0, pos.1 + dir.1);
                if grid.get(&neighbor).filter(|x| **x == '@').is_some() {
                    neighbors += 1;
                }
            }

            if neighbors < 4 {
                removed.insert(pos);
            }
        }

        let removed_count = removed.len();

        for pos in removed {
            grid.set(&pos, '.');
        }

        res2 += removed_count;
        if removed_count == 0 {
            break;
        }
    }

    println!("{res1} {res2}");
}
