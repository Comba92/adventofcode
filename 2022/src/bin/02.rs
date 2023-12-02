#[derive(Clone, Copy)]
enum Opponent {
  Rock = 'A' as isize, Paper = 'B' as isize, Scissors = 'C' as isize
}

#[derive(Clone, Copy)]
enum Player {
  Rock = 'X' as isize, Paper = 'Y' as isize, Scissors = 'Z' as isize
}


fn fight(opponent: &Opponent, player: &Player) -> i32 {
  match (&opponent, &player) {
    (Opponent::Rock, Player::Rock)     => 3 + *player as i32,
    (Opponent::Rock, Player::Paper)    => 6 + *player as i32,
    (Opponent::Rock, Player::Scissors) => 0 + *player as i32,

    (Opponent::Paper, Player::Paper)    => 3 + *player as i32,
    (Opponent::Paper, Player::Rock)     => 0 + *player as i32,
    (Opponent::Paper, Player::Scissors) => 6 + *player as i32,

    (Opponent::Scissors, Player::Scissors) => 3 + *player as i32,
    (Opponent::Scissors, Player::Rock)     => 6 + *player as i32,
    (Opponent::Scissors, Player::Paper)    => 0 + *player as i32,
  }
}

fn main() {
  let input = String::from(include_str!("02.txt"));  

  let result = input.lines().fold(0, |score, line| {
    let moves: Vec<&str> = line.split(' ').collect();
    
    let opponent = moves[0].chars().nth(0).unwrap();
    let player = moves[1].chars().nth(0).unwrap();

    let points = fight(&opponent, &player);

    score + points
  });

  println!("{result}");
}