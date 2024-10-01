#[allow(dead_code)]

fn fight1((opponent, player): (char, char)) -> u32 {
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

fn fight2((opponent, player): (char, char)) -> u32 {
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

  let rounds: Vec<(char, char)> = input.lines()
  .map(|line| {
    let moves: Vec<char> = line.chars().filter(|&c| c != ' ').collect();
    (moves[0], moves[1])
  }).collect();

  let result1: u32 = rounds.iter()
    .map(|&round| fight1(round))
    .sum();

  let result2: u32 = rounds.iter()
    .map(|&round| fight2(round))
    .sum();

  println!("{result1} {result2}");
}