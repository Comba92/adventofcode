use std::{collections::HashMap, hash::Hash, ops::Mul};

fn main() {
  let input = include_str!("11.txt");
  smart(input);
}

fn smart(input: &str) {
  fn map_add<K: Hash + Eq>(map: &mut HashMap<K, usize>, key: K, count: usize) {
    map.entry(key)
      .and_modify(|e| { *e += count; })
      .or_insert(count);
  }

  let mut stones = input
  .split_whitespace()
  .map(|n| (n.to_string(), 1usize))
  .collect::<HashMap<_, _>>();

  for _ in 0..75 {
    let mut curr = HashMap::new();

    for (stone, &count) in &stones {
      if stone == "0" {
        map_add(&mut curr, "1".to_string(), count);
      } else if stone.len() % 2 == 0 {
        let (left, mut right) = stone.split_at(stone.len() / 2);
        right = right.trim_start_matches("0");
        if right.is_empty() { right = "0"; }

        map_add(&mut curr, left.to_string(), count);
        map_add(&mut curr, right.to_string(), count);
      } else {
        let val = stone.parse::<u64>().unwrap().mul(2024).to_string();
        map_add(&mut curr, val, count);
      }
    }

    stones = curr;
  }

  let res = stones.values().copied().sum::<usize>();
  println!("{res}");
}

fn naive(input: &str) {
  let mut stones = input
    .split_whitespace()
    .map(|n| n.to_string())
    .collect::<Vec<_>>();

  for _ in 0..25 {
    let mut curr = Vec::new();

    for stone in &stones {
      if stone == "0" {
        curr.push(String::from("1"));
      } else if stone.len() % 2 == 0 {
        let (left, mut right) = stone.split_at(stone.len() / 2);
        right = right.trim_start_matches("0");
        if right.is_empty() { right = "0"; }

        curr.push(left.to_string());        
        curr.push(right.to_string());        
      } else {
        let val = stone.parse::<u64>().unwrap().mul(2024).to_string();
        curr.push(val);
      }
    }

    stones = curr;
  }

  println!("{}", stones.len());
}