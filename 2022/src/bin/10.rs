fn main() {
  let input = include_str!("10.txt");
  
  let mut x = 1;
  let mut to_add: Option<i32> = None;
  let mut cycles: usize = 0;

  let mut result1 = 0;
  for line in input.lines() {
    let instruction = line.split_whitespace().collect::<Vec<_>>();


    match instruction.as_slice() {
      ["noop"] => {

      },
      ["addx", value] => {

      },
      _ => unreachable!()
    };

    // if (cycles > 20) && ((cycles - 20) % 40 == 0) {
    //   println!("{} * {} = {}", x, cycles, x * cycles as i32);
    //   result1 += x * cycles as i32;
    // }
  }
  
  println!("{result1}");
}