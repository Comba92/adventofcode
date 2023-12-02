fn main() {
  let input = String::from(include_str!("01.txt"));

  let time = std::time::SystemTime::now();
  let result = input.lines()
    .fold(0, |sum, line| {

      let line = line
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
        .replace("zero", "zero0zero");


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


  println!("{:?}", time.elapsed().unwrap());
  print!("{result}");
}
