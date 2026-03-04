fn main() {
  let input = include_str!("06.txt");
  
  let mut lines = input.lines(); 
  let operations = lines
    .next_back().unwrap()
    .split_whitespace()
    .collect::<Vec<_>>();

  let mut results1 = lines
    .next().unwrap()
    .split_whitespace()
    .map(|n| n.parse::<u64>().unwrap())
    .collect::<Vec<_>>();

  let mut results2 = results1.clone();
  
  for line in lines {
    let nums = line
      .split_whitespace()
      .map(|n| n.parse::<u64>().unwrap());

    for (i, n) in nums.enumerate() {
      match operations[i] {
        "+" => results1[i] += n,
        "*" => results1[i] *= n,
        _ => unreachable!()
      }
    }
  }

  let sum1 = results1.iter().sum::<u64>();
  println!("{sum1}");
}