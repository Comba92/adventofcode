use std::{collections::{BTreeMap, BTreeSet}, iter};

fn main() {
  let input = include_str!("09.txt").trim();
  let mut decoded1 = Vec::new();

  let mut file_blocks = Vec::new();
  let mut free_blocks = BTreeMap::new();
  
  for (i, c) in input.char_indices() {
    let n = c.to_digit(10).unwrap() as usize;

    let vals = if i % 2 == 0 {
      file_blocks.push((decoded1.len(), n));
      iter::repeat_n(i as i32 / 2, n)
    } else {
      if n != 0 {
        free_blocks.entry(n)
          .and_modify(|e: &mut BTreeSet<usize>| { e.insert(decoded1.len()); })
          .or_insert(BTreeSet::from([decoded1.len()]));
      }

      iter::repeat_n(-1, n)
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

  // println!("{decoded2:?}");
  // println!("{}", decoded2.len());
  // println!("Free blocks: {free_blocks:?} - Files: {file_blocks:?}");

  for (file_start, file_len) in file_blocks.iter().rev().copied() {
    let mut best_free = free_blocks.range(file_len..);

    if let Some((block_len, _)) = best_free.next() {
      let block_len = *block_len;

      let blocks = free_blocks.get_mut(&block_len).unwrap();
      let block_start = blocks.iter().find(|block_start| **block_start < file_start).copied().unwrap();
      let file_id = decoded2[file_start];
      
      decoded2
      .splice(block_start..block_start+file_len, iter::repeat_n(file_id, file_len));
      decoded2
      .splice(file_start..file_start+file_len, iter::repeat_n(-1, file_len));
    
      blocks.remove(&block_start);
      // println!("{file_id}");

      if blocks.is_empty() {
        free_blocks.remove(&block_len);
      }
      
      if file_len < block_len {
        let new_block_start = block_start + file_len;
        
        free_blocks.entry(block_len - file_len)
          .and_modify(|e: &mut BTreeSet<usize>| { e.insert(new_block_start); })
          .or_insert(BTreeSet::from([new_block_start]));
      }

      // println!("{free_blocks:?}");
    }
  }

  // println!();
  // println!("{decoded2:?}");
  // println!("{}", decoded2.len());
  // println!("{free_blocks:?} {file_blocks:?}");

  let res1 = checksum(&decoded1);
  // 8484226147322 - too high
  let res2 = checksum(&decoded2);

  println!("{res1}");
  println!("{res2}");
}

fn checksum(list: &[i32]) -> usize {
  list.iter()
    .enumerate()
    .filter(|(_, n)| **n != -1)
    .map(|(i, n)| *n as usize  * i)
    .sum::<usize>()
}