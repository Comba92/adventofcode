use std::fmt::Write;

fn main() {
  let input = include_str!("02.txt");

  let mut buf = String::new();

  let mut sum1 = 0;
  let mut sum2 = 0;
  for interval in input.split(',') {
    let (left, right) = interval.split_once('-').unwrap();
    let left = left.trim().parse::<u64>().unwrap();
    let right = right.trim().parse::<u64>().unwrap();

    for n in left..=right {
      buf.clear();
      write!(&mut buf, "{n}").unwrap();
      let (l, r) = buf.split_at(buf.len() / 2);
      if l == r { sum1 += n; }
    }

    for n in left..=right {
      buf.clear();
      write!(&mut buf, "{n}").unwrap();
      
      'check: for len in 1..=buf.len()/2 {
        let bytes = buf.as_bytes();
        for (a, b) in bytes.chunks(len)
          .zip(bytes.chunks(len).skip(1))
        {
          if a != b {
            continue 'check;
          }
        }

        sum2 += n;
        break 'check;
      }
    }
  }

  println!("{sum1} {sum2}");
}