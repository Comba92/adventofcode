use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
  let input = String::from(include_str!("01.txt"));

  let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::with_capacity(4);

  input.split("\r\n\r\n")
    .for_each(|group| {
      let sum = group.lines()
        .fold(0, |sum, item| {
          sum + item.parse::<i32>().unwrap()
        });

      heap.push(Reverse(sum));
      if heap.len() == 4 {
        heap.pop();
      }
    });
  

  let result: i32 = heap.iter().fold(0, |sum, rev| sum + rev.0);
  println!("{result}");
}