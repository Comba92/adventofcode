use std::{collections::HashSet, fs};


fn recursion(card: usize, matches: &Vec<usize>) -> usize {
  if card >= matches.len() { return 0 }

  let this_match = matches[card];
  let mut sum = 1;

  for next in 0..this_match {
    sum += recursion(card + next + 1, matches);
  }

  sum
}

fn main() {
  let input = include_str!("04.txt");
  let input = fs::read_to_string("C:\\Users\\Comba\\Desktop\\HxE7.txt\\bigboy.txt").unwrap();

  let mut matches_per_card = Vec::new();
  let timer = std::time::SystemTime::now();

  let result1 = input.lines().fold(0, |total_sum, line| {
    let row = line
    .split(':').nth(1).unwrap()
    .split('|').map(|list| {
      list
      .split(' ')
      .filter(|token| !token.is_empty())
      .map(|token| token.parse::<u32>().unwrap())
      .collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let mut winning = HashSet::new();
    row[0].iter().for_each(|&num| { winning.insert(num); });

    let mut row_matches = 0;
    let row_score = row[1].iter().fold(0, |sum, num| {
      if let Some(_) = winning.get(num) {
        row_matches += 1;
        if sum == 0 { 1 } else { sum * 2 }
      } else { sum }
    });

    matches_per_card.push(row_matches);

    total_sum + row_score
  });


  let result2 = (0 .. matches_per_card.len())
    .fold(0, |sum, card| 
      sum + recursion(card, &matches_per_card)
    );

  println!("{:?}", timer.elapsed().unwrap());
  println!("{result2}");
}