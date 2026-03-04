mod utils;
use utils::vector::{Vector, LEFT, RIGHT};

const DOWN: Vector = (0, 1);

static VERTICAL: [u8; 1] = [0b00011110];
static CROSS: [u8; 3] = [
    0b0000100,
    0b0001110,
    0b0000100
];
static TRIANGLE: [u8; 3] = [
    0b0000100,
    0b0001110,
    0b0000100
];
static HORIZONTAL: [u8; 4] = [
    0b0001000,
    0b0001000,
    0b0001000,
    0b0001000,
];
static SQUARE: [u8; 2] = [
    0b0001100,
    0b0001100
];

static TO_SPAWN: [&[u8]; 5] = [
    &VERTICAL, &CROSS, &TRIANGLE, &HORIZONTAL, &SQUARE
];

fn draw(grid: &Vec<u8>) {
    for row in grid.iter().rev() {
        for i in 0..7 {
            // 64 is the second bit from the right
            let col = row & (64 >> i);
            if col == 0 { print!("."); }
            else { print!("#"); }
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
  
    let mut grid: Vec<u8> = Vec::from([0, 0, 0, 0]);

    let mut spawn = TO_SPAWN.iter().enumerate().cycle();
    let mut wind = pattern.iter().enumerate().cycle();

    for _ in 0..1 {
        let current = spawn.next().unwrap();


        draw(&grid);
    }

}