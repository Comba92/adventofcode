use std::{collections::{BTreeMap, BTreeSet}, iter};

fn main() {
  let input = include_str!("09.txt").trim();

  let map = input.chars()
    .map(|c| c.to_digit(10).unwrap() as isize);

  // get file blocks lengths list
  let files = map.clone()
    .enumerate()
    .filter_map(|b| (b.0 % 2 == 0).then_some(b.1))
    .collect::<Vec<_>>();

  // get empty blocks lengths list
  let empties = map.clone()
    .enumerate()
    .filter_map(|b| (b.0 % 2 == 1).then_some(b.1))
    .collect::<Vec<_>>();

  let mut mem = Vec::new();

  let mut file_blocks_count = 0;
  // first pass: add file_blocks to disk
  // iter files togheter with empty spaces
  for (id, (file_len, empty_len)) in files.iter()
    .zip(empties.iter())
    .enumerate()
  {
    let file_blocks = iter::repeat_n(id as isize, *file_len as usize);
    mem.extend(file_blocks);
    file_blocks_count += *file_len;

    let empty_blocks = iter::repeat_n(-1, *empty_len as usize);
    mem.extend(empty_blocks);
  }
  // push last
  let last = *files.last().unwrap();
  let file_blocks = iter::repeat_n(files.len() as isize-1, last as usize);
  mem.extend(file_blocks);
  file_blocks_count += last;

  
  let mut files_clone = files.clone();
  let mut mem1 = mem.clone();

  // second pass: add empty_blocks to disk
  for block in mem1
    .iter_mut()
    .filter(|b| **b == -1)
  {
    if files_clone.is_empty() { break; }

    let id = files_clone.len() as isize - 1;
    let last = files_clone.last_mut().unwrap();

    *block = id;
    *last -= 1;
    if *last <= 0 { files_clone.pop(); }
  }
  mem1.truncate(file_blocks_count as usize);


  let mut file_starts = Vec::new();
  let mut empty_blocks: BTreeMap<isize, BTreeSet<usize>> = BTreeMap::new();

  let mut cumsum = 0;
  // build file_blocks<index, length> list
  // build empty_blocks<length, Set<indexes>> map 
  for (file_len, empty_len) in map.clone()
    .step_by(2)
    .zip(map.skip(1).step_by(2))
  {
    file_starts.push(cumsum);
    cumsum += file_len as usize;

    if empty_len > 0 {
      empty_blocks.entry(empty_len)
        .or_default()
        .insert(cumsum);
    }

    cumsum += empty_len as usize;
  }
  // push last
  file_starts.push(cumsum);

  let mut mem2 = mem.clone();
  for (file_id, file_len) in files
    .iter()
    .enumerate()
    .rev()
  {
    // if there is any free space of file_len
    let free_blocks = empty_blocks
      .range_mut(file_len..)
      .skip_while(|entry| entry.1.is_empty())
      .next();

    let mut should_update = None;
    if let Some((empty_len, free_blocks)) = free_blocks {
      // never move a file to the right
      if *free_blocks.first().unwrap() > file_starts[file_id] { continue; }
      
      let empty_block_start = free_blocks.pop_first().unwrap() as usize;

      // add left
      mem2[empty_block_start .. empty_block_start+ *file_len as usize].fill(file_id as isize);
      
      // remove right
      let file_block_start = file_starts[file_id];
      mem2[file_block_start..file_block_start+*file_len as usize].fill(-1);
      
      println!("Moving {file_id} from {file_block_start} to {empty_block_start}");
      should_update = Some((empty_block_start, *empty_len));
    }

    if let Some((old_empty_start, old_empty_len)) = should_update {
      let new_empty_len = old_empty_len - *file_len;
      let new_empty_start = old_empty_start + (old_empty_len - new_empty_len) as usize;
      
      if new_empty_len > 0 {
        empty_blocks.entry(new_empty_len)
          .or_default()
          .insert(new_empty_start);
      }
    }

    println!("{empty_blocks:?}");
  }

  let res1 = checksum(&mem1);
  let res2 = checksum(&mem2);
  println!("{res1:?} {res2:?}");

  // println!("{mem2:?}");
}

fn checksum(blocks: &[isize]) -> isize {
  blocks.iter()
    .enumerate()
    .filter(|(_, v)| **v != -1)
    .map(|(i, &v)| i as isize * v)
    .sum()
}

// 6297995201652 too low
// 8484226147322 too high