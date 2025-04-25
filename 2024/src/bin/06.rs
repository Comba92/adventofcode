

fn main() {
  let input = include_str!("06.txt");

  let matrix = input
  .lines()
  .map(|line| line.chars().collect::<Vec<_>>())
  .collect::<Vec<_>>();
}