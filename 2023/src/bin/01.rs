const NUMS: [(&str, u32); 20] = [
  ("one", 1),
  ("two", 2),
  ("three", 3),
  ("four", 4),
  ("five", 5),
  ("six", 6),
  ("seven", 7),
  ("eight", 8),
  ("nine", 9),
  ("zero", 0),
  ("1", 1),
  ("2", 2),
  ("3", 3),
  ("4", 4),
  ("5", 5),
  ("6", 6),
  ("7", 7),
  ("8", 8),
  ("9", 9),
  ("0", 0)
];

fn main() {
  let input = String::from(include_str!("01.txt"));

  let result = input.lines()
    .fold(0, |sum, line| {
      let left_digit = line
        .chars()
        .find(|c| c.is_ascii_digit());
      let right_digit = line
        .chars()
        .rfind(|c| c.is_ascii_digit());


      if left_digit.is_some() && right_digit.is_some() {
        let left_digit = left_digit.unwrap().to_digit(10).unwrap();
        let right_digit = right_digit.unwrap().to_digit(10).unwrap();
        sum + left_digit * 10 + right_digit 
      } else { sum }
    });

  print!("{result}");
}
