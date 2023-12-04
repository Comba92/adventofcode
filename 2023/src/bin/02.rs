#![allow(dead_code, unused)]

fn count_and_color_to_tuple(s: &str) -> (usize, &str) {
  let mut group = s
  .split(' ')
  .filter(|token| !token.is_empty());
  
  let count = group.next().unwrap().parse().unwrap();
  let color = group.next().unwrap();
  (count, color)
}

use std::fs;

fn main() {
  let input = include_str!("02.txt");

  let timer = std::time::SystemTime::now();
  let mut games = Vec::new();

  input.lines().for_each(|game| {
    let extractions: Vec<_> = game
      // skip game ID
      .split(':').nth(1).unwrap()
      // split per extraction
      .split(';').collect();
       

    let mut maxes = [0; 3];

    extractions.iter().for_each(|extraction| {
      // split per cube
      extraction.split(',')
        .map(count_and_color_to_tuple)
        .for_each(|(quantity, color)| {
          match color {
            "red" => maxes[0] = maxes[0].max(quantity),
            "green" => maxes[1] = maxes[1].max(quantity),
            "blue" => maxes[2] = maxes[2].max(quantity),
            _ => unreachable!(),
          };
        });
      });

      games.push(maxes);
  });

  let result1: usize = games.iter()
    .enumerate()
    .filter(|game| {
      game.1[0] <= 12 && game.1[1] <= 13 && game.1[2] <= 14
    })
    .map(|game| game.0 + 1)
    .sum();

  let result2: usize = games.iter()
    .map(|game| game.iter().product::<usize>())
    .sum();


  println!("{:?}", timer.elapsed().unwrap());
  println!("{result2}");
}
