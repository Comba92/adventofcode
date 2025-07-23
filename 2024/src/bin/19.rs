use std::collections::{HashMap, HashSet};

fn main() {
  let input = include_str!("19.txt");

  let (patterns, designs) = input.split_once("\r\n\r\n").unwrap();

  let patterns = patterns
    .split_whitespace()
    .map(|s| s.strip_suffix(',').unwrap_or(s))
    .fold(HashSet::new(), |mut s, p| {
      s.insert(p);
      s
    });

  println!("{patterns:?}");

  let ptrn_max_len = patterns.iter()
    .map(|s| s.len())
    .max().unwrap();

  let mut memo = HashMap::new();

  let mut res1 = 0;
  let mut res2 = 0;

  for design in designs.lines() {
    let rec = solve_rec2(&patterns, ptrn_max_len, design, &mut memo);
    if rec > 0 {
      res1 += 1;
    }
    res2 += rec;
  }

  println!("{res1} {res2}");
}

fn solve_rec1(patterns: &HashSet<&str>, ptrn_max_len: usize, design: &str) -> bool {
  if design.is_empty() { return true; }

  let max_len = ptrn_max_len.min(design.len());
  for len in 1..=max_len {
    if patterns.contains(&design[0..len]) {
      // println!("{design}\tcontains {}", &design[0..len]);
      if solve_rec1(patterns, ptrn_max_len, &design[len..]) {
        return true;
      }
    }
  }

  return false;
}

fn solve_rec2<'a>(patterns: &HashSet<&str>, ptrn_max_len: usize, design: &'a str, memo: &mut HashMap<&'a str, usize>) -> usize {
  if design.is_empty() { return 1; }

  let max_len = ptrn_max_len.min(design.len());
  let mut options = 0;
  for len in 1..=max_len {
    if patterns.contains(&design[0..len]) {      
      let slice = &design[len..];
      if let Some(count) = memo.get(slice) {
        options += count;
      } else {
        let rec = solve_rec2(patterns, ptrn_max_len, slice, memo);
        options += rec;
        memo.insert(slice, rec);
      }
    }
  }

  return options;
}