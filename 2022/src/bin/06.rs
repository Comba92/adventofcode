use std::collections::HashSet;

fn solve(input: &str, window_size: usize) -> usize {
  for (i, window) in input.as_bytes().windows(window_size).enumerate() {
    let set: HashSet<&u8> = HashSet::from_iter(window.iter());
    if set.len() == window_size {
      return i + window_size;
    }
  }

  unreachable!();
}

fn main() {
  let input = include_str!("06.txt");

  let result1 = solve(input, 4);
  let result2 = solve(input, 14);

  println!("{result1} {result2}");
}