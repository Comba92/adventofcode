#[derive(Debug)]
struct Machine {
  a: (i64, i64),
  b: (i64, i64),
  p: (i64, i64)
}

fn main() {
  let input = include_str!("13.txt");
  let input = input.replace("\r\n", "\n");

  let machines = input
    .split("\n\n")
    .map(|s| {
      let mut lines = s.lines();

      let parse_button = |line: &str, sep: char| {
        let x = line.chars()
          .skip_while(|c| *c != sep)
          .skip(1)
          .take_while(|c| *c != ',')
          .collect::<String>()
          .parse::<i64>()
          .unwrap();

        let y = line.chars()
          .skip_while(|c| *c != sep)
          .skip(1)
          .skip_while(|c| *c != sep)
          .skip(1)
          .take_while(|c| c.is_numeric())
          .collect::<String>()
          .parse::<i64>()
          .unwrap();

        (x, y)
      };

      let a = parse_button(lines.next().unwrap(), '+');
      let b = parse_button(lines.next().unwrap(), '+');
      let p = parse_button(lines.next().unwrap(), '=');

      Machine { a, b, p }
    })
    .collect::<Vec<_>>();

    // ax*ap + bx*bp = px
    // ay*ap + by*bp = py
    // ap = (px - bx*bp)/ax
    // ap = (py - by*bp)/ay
    // (px - bx*bp)/ax = (py - by*bp)/ay
    // ay(px - bx*bp) - ax(py - by*bp) = 0
    // ay*px - ay*bx*bp - ax*py + ax*by*bp = 0
    // bp (ax*by - ay*bx) = ax*py - ay*px
    // bp = (ax*py - ay*px) / (ax*by - ay*bx)

  let res1 = machines
    .iter()
    .map(|m| {
      let bp = (m.a.0 * m.p.1 - m.a.1 * m.p.0) / (m.a.0 * m.b.1 - m.a.1 * m.b.0);
      let ap = (m.p.0 - m.b.0 * bp) / m.a.0;

      let xok = m.a.0 * ap + m.b.0 * bp == m.p.0;
      let yok = m.a.1 * ap + m.b.1 * bp == m.p.1;

      if !xok || !yok || ap >= 100 || bp >= 100 {
        0
      } else {
        ap * 3 + bp
      }
    })
    .sum::<i64>();

  let res2 = machines
    .iter()
    .map(|m| {
      let px = m.p.0 + 10000000000000;
      let py = m.p.1 + 10000000000000;
      let bp = (m.a.0 * py - m.a.1 * px) / (m.a.0 * m.b.1 - m.a.1 * m.b.0);
      let ap = (px - m.b.0 * bp) / m.a.0;

      let xok = m.a.0 * ap + m.b.0 * bp == px;
      let yok = m.a.1 * ap + m.b.1 * bp == py;

      if !xok || !yok {
        0
      } else {
        ap * 3 + bp
      }
    })
    .sum::<i64>();

  println!("{res1:?} {res2:?}");
}


// this is too slow, and naive.
// fn solve_rec(m: &Machine, (x, y): (i64, i64), tokens_spent: i64, presses: i64) -> Option<i64> {
//   if x == m.p.0 && y == m.p.1 { return Some(tokens_spent); }
//   if x > m.p.0 || y > m.p.1 || presses > 100 { return None; }

//   println!("{x} {y} {presses}");

//   let a = solve_rec(m, (x + m.a.0, y + m.a.1), tokens_spent + 3, presses+1);
//   let b = solve_rec(m, (x + m.b.0, y + m.b.1), tokens_spent + 1, presses+1);
//   return a.min(b)
// }