use std::collections::BinaryHeap;

fn main() {
  let input = String::from(include_str!("01.txt"));

  let mut heap: BinaryHeap<u32> = BinaryHeap::new();

  let groups = input.split("\r\n\r\n");

  for group in groups {
    let sum = group.lines()
      .map(|item| item.parse::<u32>().unwrap())
      .sum();

    heap.push(sum);
  }
  
  let result1 = heap.peek().unwrap().to_owned();
  let result2: u32 = heap.into_sorted_vec().iter().rev().take(3).sum();
  
  println!("{result1} {result2}");
}