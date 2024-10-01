
fn main() {
  let mut input = include_str!("05.txt").split("\r\n\r\n");

  let crates = input.next().unwrap();
  let stacks_count = crates.lines().last().unwrap().as_bytes().chunks(4).len();
  let mut stacks: Vec<Vec<char>> = (0..stacks_count).map(|_| Vec::new()).collect();

  // Build matrix
  for line in crates.lines().rev().skip(1) {
    let row = line.as_bytes().chunks(4).enumerate();
    for (col, krate) in row {
      let chars = krate.iter().filter(|c| c.is_ascii_alphabetic()).collect::<Vec<_>>();
      if !chars.is_empty() { stacks[col].push(chars[0].to_owned() as char); }
    }
  }


  let mut stacks1 = stacks.clone();
  let mut stacks2 = stacks.clone();
  for line in input.next().unwrap().lines() {
    let nums: Vec<usize> = line.split(' ')
    .filter(|s| s.parse::<usize>().is_ok())
    .map(|s| s.parse().unwrap()).collect();

    let moves = nums[0];
    let from = nums[1] - 1;
    let to = nums[2] - 1;

    for _ in 0..moves {
      let krate = stacks1[from].pop().unwrap();
      stacks1[to].push(krate);
    }

    let pop_start = stacks2[from].len()-moves;
    let mut popped: Vec<char> = stacks2[from].drain(pop_start..).collect();
    stacks2[to].append(&mut popped);
  }

  let result1: String = stacks1.iter().map(|stack| stack.last().unwrap()).collect();
  let result2: String = stacks2.iter().map(|stack| stack.last().unwrap()).collect();

  println!("{result1} {result2}");
}