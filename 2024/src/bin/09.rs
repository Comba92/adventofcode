use std::{collections::BTreeMap, iter};

// 6288338133779 - too low
// 6288599497374 - too high
fn main() {
  let input = include_str!("09.txt");
  let mut decoded1 = Vec::new();

  let mut file_blocks = Vec::new();
  let mut free_blocks = BTreeMap::new();
  
  for (i, c) in input.char_indices() {
    let n = c.to_digit(10).unwrap() as usize;

    let vals = if i % 2 == 0 {
      file_blocks.push((decoded1.len(), n));
      iter::repeat(i as i32 / 2).take(n)
    } else {
      free_blocks.insert(decoded1.len(), n);
      iter::repeat(-1).take(n)
    };

    decoded1.extend(vals);
  }

  let mut decoded2 = decoded1.clone();

  let mut right = decoded1.len()-1;
  while decoded1[right] == -1 { right -= 1; }

  for left in 0..decoded1.len() {
    if decoded1[left] == -1 {
      decoded1.swap(left, right);
      right -= 1;
      while decoded1[right] == -1 { right -= 1; }
    }

    if left >= right { break; }
  }

  let res1 = decoded1.iter()
    .enumerate()
    .filter(|(_, n)| **n != -1)
    .map(|(i, n)| *n as usize  * i)
    .sum::<usize>();

  println!("{res1}");
}