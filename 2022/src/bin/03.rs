use std::collections::HashSet;

fn get_priority(item: &char) -> u32 {
  match item {
    'a'..='z' => *item as u32 - 'a' as u32 + 1,
    'A'..='Z' => *item as u32 - 'A' as u32 + 27,
    _ => unreachable!()
  }
} 

fn main() {
  let input = include_str!("03.txt");
  
  let mut result1 = 0;
  for line in input.lines() {
    let compartments = line.split_at(line.len() / 2);

    let set1: HashSet<char> = HashSet::from_iter(compartments.0.chars());
    let set2: HashSet<char> = HashSet::from_iter(compartments.1.chars());
    let intersection = set1.intersection(&set2);

    let sum: u32 = intersection.map(get_priority).sum();
    result1 += sum;
  }


  let mut result2 = 0;
  const GROUP_LEN: usize = 3;
  let lines = input.lines().collect::<Vec<_>>();

  for group in lines.chunks(GROUP_LEN) {
    let mut sets: Vec<HashSet<char>> = Vec::new();

    for line in group {
      let set: HashSet<char> = HashSet::from_iter(line.chars());
      sets.push(set);
    }

    // This computes the intersection of n sets. Smart!
    let sum: u32 = sets[0].iter()
    .filter(|c| sets.iter().all(|set| set.contains(c)))
    .map(get_priority).sum();

    result2 += sum;
  }

  println!("{result1} {result2}");
}