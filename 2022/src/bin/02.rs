#[allow(dead_code)]

fn fight_1(opponent: char, player: char) -> u32 {
  match (opponent, player) {
    ('A', 'X') => 3 + 1,
    ('A', 'Y') => 6 + 2,
    ('A', 'Z') => 0 + 3,
    ('B', 'Y') => 3 + 2,
    ('B', 'X') => 0 + 1,
    ('B', 'Z') => 6 + 3,
    ('C', 'Z') => 3 + 3,
    ('C', 'X') => 6 + 1,
    ('C', 'Y') => 0 + 2,
    _ => panic!("shouldnt happen")
  }
}

fn fight_2(opponent: char, player: char) -> u32 {
  match (opponent, player) {
    ('A', 'X') => 0 + 3,
    ('A', 'Y') => 3 + 1,
    ('A', 'Z') => 6 + 2,
    ('B', 'X') => 0 + 1,
    ('B', 'Y') => 3 + 2,
    ('B', 'Z') => 6 + 3,
    ('C', 'X') => 0 + 2,
    ('C', 'Y') => 3 + 3,
    ('C', 'Z') => 6 + 1,
    _ => panic!("shouldnt happen")
  }
}


fn main() {
  let input = String::from(include_str!("02.txt"));  

  let result = input.lines().fold(0, |score, line| {
    let moves: Vec<&str> = line.split(' ').collect();
    
    let opponent = moves[0].chars().nth(0).unwrap();
    let player = moves[1].chars().nth(0).unwrap();

    let points = fight_2(opponent, player);

    score + points
  });

  println!("{result}");
}