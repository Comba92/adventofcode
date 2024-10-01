use std::ops::RangeBounds;

#[derive(Debug)]
struct Mapping {
  dst_start: u64,
  src_start: u64,
  range_len: u64
}

fn main() {
  let input = include_str!("05.txt");

  let mut line_iter = input.lines()
  .peekable();

  let seeds: Vec<u64> = line_iter.next().unwrap()
  .split(|c: char| c.is_whitespace())
  .filter(|token| !token.is_empty())
  .skip(1)
  .map(|n| n.parse().unwrap())
  .collect();

  let mut maps = Vec::new();

  // .next() skips the empty line
  while line_iter.next().is_some() {
    // skip the header
    line_iter.next();
    
    let mut group_map = Vec::new();
    
    while line_iter.peek().is_some() 
    && line_iter.peek().unwrap().contains(|c: char| c.is_ascii_digit()) {
      let row_map: Vec<u64> = line_iter.next().unwrap()
      .split(|c: char| c.is_whitespace()).filter(|token| !token.is_empty())
      .map(|num| num.parse().unwrap())
      .collect();

      group_map.push(Mapping {
        dst_start: row_map[0],
        src_start: row_map[1],
        range_len: row_map[2]
      });
    }

    if !group_map.is_empty() { maps.push(group_map); }
  }

  // dst_start + current - src_start
  let result1 = seeds.iter()
  .map(|seed| {
    let mut next_seed = seed.clone();
    for map_group in maps.iter() {
      for map in map_group.iter() {
        if (map.src_start .. map.src_start + map.range_len).contains(&next_seed) {
          next_seed = map.dst_start + next_seed - map.src_start;
          break;
        }
      }
    }
    
    next_seed
  }).min().unwrap();

  let result2 = seeds.chunks_exact(2)
  .map(|chunk| (chunk[0], chunk[1]))
  .map(|(seed_start, seed_end)| {
      let mut old_seeds = vec!((seed_start, seed_end));

      for map_group in maps.iter() {
        let mut new_seeds = Vec::new();
        for map in map_group.iter() {
          let range = (map.src_start, map.src_start + map.range_len);

          
        }
      }
    });
}