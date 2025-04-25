use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}};

fn main() {
  let input = include_str!("01.txt");

  let parsed = input
    .lines()
    .map(|line| line.split_whitespace())
    .map(|mut pair| (pair.next().unwrap(), pair.next().unwrap()))
    .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
    .map(|(a, b)| (Reverse(a), Reverse(b)));

  let mut heaps: (BinaryHeap<_>, BinaryHeap<_>) = parsed.clone().unzip();

  let mut res1 = 0;
  while !heaps.0.is_empty() {
    let Reverse(left)  = heaps.0.pop().unwrap();
    let Reverse(right) = heaps.1.pop().unwrap();
    res1 += left.abs_diff(right);
  }

  let lists: (Vec<_>, Vec<_>) = parsed.unzip();
  let map = lists.1.into_iter()
    .fold(HashMap::new(), |mut map, n| {
      map.entry(n.0)
        .and_modify(|e| *e += 1)
        .or_insert(1);
      map
    });

  let res2 = lists.0.into_iter()
    .map(|Reverse(n)| n * map.get(&n).copied().unwrap_or(0))
    .sum::<i32>();

  println!("{res1}");
  println!("{res2}");
}