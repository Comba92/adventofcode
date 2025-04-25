fn main() {
  let input = include_str!("03.txt");

  let mut matches1 = Vec::new();
  let mut matches2 = Vec::new();
  let mut enabled = true;
  for i in 0..input.len() {
    let curr = &input[i..];

    if curr.starts_with("do()") {
      enabled = true;
    } else if curr.starts_with("don't()") {
      enabled = false;
    } else if curr.starts_with("mul(") {
      matches1.push(i);
      if enabled { matches2.push(i); }
    }
  }

  let res1 = interpret(input, matches1);
  let res2 = interpret(input, matches2);

  println!("{res1}");
  println!("{res2}");
}

fn interpret(input: &str, matches1: Vec<usize>) -> i32 {
  matches1.iter()
    .filter_map(|&start| input[start..]
      .find(')')
      .and_then(|end| Some(&input[start+4..start+end]))
    )
    .inspect(|s| println!("{s}"))
    .map(|s| s
      .trim_matches(',')
      .split_once(',')
      .map(|(a, b)| (a.parse::<i32>(), b.parse::<i32>()))
    )
    .fold(0, |acc, splitted| {
      match splitted {
        Some(values) => {
          let x = values.0.unwrap_or_default();
          let y = values.1.unwrap_or_default();
      
          acc + (x * y)
        }
        None => acc,
      }
    })
}