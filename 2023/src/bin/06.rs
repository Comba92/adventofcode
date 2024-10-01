fn main() {
  let input = include_str!("06.txt");

  let mut lines = input.lines();
  let lexer = |text: &str| text
    .split(|c: char| c.is_whitespace())
    .filter(|token| !token.is_empty())
    // skip "Time:" string
    .skip(1)
    .map(|n| n.parse().unwrap())
    .collect();

  let times: Vec<i32> = lexer(lines.next().unwrap());
  let distances: Vec<i32> = lexer(lines.next().unwrap());
  
  // distance = (time_ms - press_ms) * press_ms
  
  let timer = std::time::SystemTime::now();
  let result1 = times.iter()
    .zip(distances.iter())
    .map(|(time, distance)|
      (0..*time)
        .map(|press| (time - press) * press)
        .filter(|dist| dist > distance)
        .collect::<Vec<i32>>()
        .len())
      .fold(1, |product, way| product * way);

    let vec_to_number = |vec: Vec<i32>| vec.iter()
      .map(|n| n.to_string())
      .fold(String::new(), |n_str, s| n_str + &s)
      .parse::<i64>().unwrap();

    let big_time = vec_to_number(times);
    let big_distance = vec_to_number(distances);

    let result2 = (0..big_time)
      .map(|press| (big_time - press) * press)
      .filter(|dist| *dist > big_distance)
      .collect::<Vec<i64>>()
      .len();

    println!("{:?} {:?} in {:?}", result1, result2, timer.elapsed().unwrap());
}