fn main() {
  let input = include_str!("03.txt");

  let mut sum1 = 0;
  let mut sum2 = 0;
  for line in input.lines()
    .filter(|line| !line.is_empty())
  {
    sum1 += solve(line, 2);
    sum2 += solve(line, 12);
  }

  println!("{sum1} {sum2}");
}

fn solve(line: &str, len: usize) -> u64 {
    let iter = line.chars()
      .rev() // we reverse the iterator so that we get always the first max from the left
      .enumerate();

    let mut jolt = 0;
    let mut last_max = line.len();

    for i in (0..len).rev() {
      let digit = iter.clone()
        .skip(i) // skip last n chars as they cant be current max
        .take(last_max - i) // take until last max, without counting skipped lasts
        .max_by_key(|x| x.1)
        .unwrap();

      last_max = digit.0;
      jolt += 10u64.pow(i as u32) * digit.1.to_digit(10).unwrap() as u64;
    }

    jolt
}