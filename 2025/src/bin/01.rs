fn main() {
  const MODULE: i64 = 100;
  let input = include_str!("01.txt");

  let mut dial = 50;
  let mut zeroes1 = 0;
  let mut zeroes2 = 0;
  for line in input.lines().map(|l| l.as_bytes()) {
    let amt = str::from_utf8(&line[1..])
    .unwrap()
      .parse::<i64>()
      .unwrap();
    let amt_trim = amt % 100;
    
    let dir = line[0];
    if dir == b'L' {
      if dial != 0 && dial - amt_trim <= 0 {
        zeroes2 += 1;
      }

      dial = (dial - amt).rem_euclid(MODULE);
    } else if dir == b'R' {
      if dial + amt_trim >= 100 {
        zeroes2 += 1;
      }

      dial = (dial + amt).rem_euclid(MODULE);
    } else {
      unreachable!()
    }

    if dial == 0 { zeroes1 += 1; }
    zeroes2 += amt / MODULE;
  }

  println!("Res: {} {}", zeroes1, zeroes2);
}