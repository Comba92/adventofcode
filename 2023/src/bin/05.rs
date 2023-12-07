fn recursion_silver(current: i64, group: usize, maps: &Vec<Vec<Vec<i64>>>) -> i64 {
  if group >= maps.len() { return current; }

  let mut minimum = i64::MAX;

  maps[group].iter().enumerate().for_each(|(i, row)| {
    let dst_start = row[0];
    let src_start = row[1];
    let range_len = row[2];

    if src_start <= current && current < src_start + range_len {
      let next = dst_start + current - src_start;
      let rec = recursion_silver(next, group+1, maps);
      minimum = minimum.min(rec);
    }
  });

  if minimum == i64::MAX { recursion_silver(current, group+1, maps) } else { minimum }
}

fn recursion(seed: (i64, i64), group: usize, maps: &Vec<Vec<Vec<i64>>>) -> i64 {
  if group == maps.len() {
    return seed.0;
  }

  println!("{:?}", seed);
  let mut minimum = i64::MAX;

  maps[group].iter()
  .filter(|row| !((seed.1 < row[1]) || (seed.0 > row[1] + row[2])))
  .for_each(|row| {
    let dst_start = row[0];
    let src_start = row[1];
    let src_len = row[2];

    let src_end = src_start + src_len - 1;

    let center = (src_start.max(seed.0), src_end.min(seed.1));
    let left = (seed.0, center.0 - 1);
    let right = (center.1 + 1, seed.1);
    
    if left.0 < left.1     { minimum = minimum.min(recursion(left, group+1, maps)); }
    if right.0 < right.1   { minimum = minimum.min(recursion(right, group+1, maps)); }
    if center.0 < center.1 {
      let center = (dst_start + center.0 - src_start, dst_start + center.1 - src_start);
      minimum = minimum.min(recursion(center, group+1, maps));
    }
  });

  if minimum == i64::MAX { recursion(seed, group+1, maps) } else { minimum } 
}

fn main() {
  let input = include_str!("05.txt");

  let mut line_iter = input.lines().peekable();
  
  let mut seeds = line_iter.next().unwrap()
  .split(':').nth(1).unwrap()
  .split(' ').filter(|token| !token.is_empty())
  .map(|num| num.parse::<i64>().unwrap())
  .collect::<Vec<_>>();

  let mut maps = Vec::new();

  while line_iter.next().is_some() {
    let mut group_map = Vec::new();
    
    while line_iter.peek().is_some() 
    && line_iter.peek().unwrap().contains(|c: char| c.is_ascii_digit()) {
      let row_map = line_iter.next().unwrap()
      .split(' ').filter(|token| !token.is_empty())
      .map(|num| num.parse::<i64>().unwrap())
      .collect::<Vec<_>>();

      group_map.push(row_map);
    }

    if !group_map.is_empty() { maps.push(group_map); }
  }

  let result1 = seeds.iter().map(|&value| {
    let mut current = value;

    maps.iter().for_each(|group| {
      for row in group {
        let dst_start = row[0];
        let src_start = row[1];
        let src_len = row[2];

        if src_start <= current && current < src_start + src_len {
          current = dst_start + current - src_start;
          break;
        }
      }
    });

    current
  }).min().unwrap();

  let timer = std::time::SystemTime::now();
  let result2 = seeds.as_slice().chunks_exact(2)
  .enumerate()
  .map(|(i, seed)| {
    println!("{i} seeds solved in {:?}...", timer.elapsed());
    (seed[0] .. seed[0] + seed[1] - 1).map(|value| {
      let mut current = value;
  
      maps.iter().for_each(|group| {
        for row in group {
          let dst_start = row[0];
          let src_start = row[1];
          let src_len = row[2];
  
          if src_start <= current && current < src_start + src_len {
            current = dst_start + current - src_start;
            break;
          }
        }
      });
  
      current
    }).min().unwrap()
  }).min().unwrap();

  println!("{result1} {result2}");
}