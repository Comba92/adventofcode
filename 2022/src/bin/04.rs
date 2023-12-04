fn main() {
  let input = include_str!("04.txt");

  let pairs = input.lines().map(|line| {
    let mut pairs = line.split(',')
    .map(|pair| pair.split('-')
      .map(|num| num.parse::<u32>().unwrap())
      .collect::<Vec<_>>()
    );
    
    (pairs.next().unwrap(), pairs.next().unwrap())
  }).collect::<Vec<_>>();

  let result1 = pairs.iter().fold(0, |sum, ranges| {
    if ranges.0[0] <= ranges.1[0] && ranges.1[1] <= ranges.0[1]
    || ranges.1[0] <= ranges.0[0] && ranges.0[1] <= ranges.1[1] 
    { sum + 1 } else { sum }
  });

  let result2 = pairs.iter().fold(0, |sum, ranges| {
    if ranges.0[0] <= ranges.1[0] && ranges.1[1] <= ranges.0[1]
    || ranges.1[0] <= ranges.0[0] && ranges.0[1] <= ranges.1[1]
    || ranges.0[1] >= ranges.1[0] && ranges.0[0] <= ranges.1[1] 
    { println!("{:?}", ranges); sum + 1 } else { sum }
  });

  println!("{result1} {result2}");
}