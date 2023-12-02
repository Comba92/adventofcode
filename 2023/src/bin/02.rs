fn count_and_color_to_tuple(s: &str) -> (usize, &str) {
  let group = s
  .split(' ')
  .filter(|token| token.len() != 0)
  .collect::<Vec<_>>();
  
  (group[0].parse().unwrap(), group[1])
}


fn main() {
  let input = String::from(include_str!("02.txt"));

  let mut games = Vec::new();

  input.lines().for_each(|game| {
    let extractions: Vec<_> = game
      // skip game ID
      .split(':').nth(1).unwrap()
      // split per extraction
      .split(';').collect();
       

    let mut maxes = [0; 3];
    let mut mins = [0; 3];

    extractions.iter().for_each(|extraction| {
      // split per cube
      extraction.split(',')
        .map(count_and_color_to_tuple)
        .for_each(|(quantity, color)| {
          match color {
            "red" => {
              maxes[0] = maxes[0].max(quantity);
              mins[0] = mins[0].min(quantity);
            },
            "green" => {
              maxes[1] = maxes[1].max(quantity);
              mins[1] = mins[1].min(quantity);
            },
            "blue" => {
              maxes[2] = maxes[2].max(quantity);
              mins[2] = mins[2].min(quantity);
            },
            _ => panic!("shouldnt happen"),
          };
        });
      });

      games.push(maxes);
  });

  #[allow(unused)]
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

  println!("{result2}");
}
