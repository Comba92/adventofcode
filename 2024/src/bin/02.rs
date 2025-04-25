fn main() {
  let input = include_str!("02.txt");

  let reports = input
    .lines()
    .map(|line| line
      .split_whitespace()
      .map(|l| l.parse::<i32>().unwrap())
      .collect::<Vec<_>>()
    );

  let mut res1 = 0;
  let mut res2 = 0;
  for mut report in reports {
    if let Some(i) = bad_report_position(&report) {
      let mut report_cloned = report.clone();
      
      // we try removing either the first or second pair
      report.remove(i);
      report_cloned.remove(i+1);
      
      if bad_report_position(&report).is_none()
      || bad_report_position(&report_cloned).is_none() {
        res2 += 1;
      }
    } else {
      res1 += 1;
      res2 += 1;
    }
  }

  println!("{res1}");
  println!("{res2}");
}

fn bad_report_position(report: &[i32]) -> Option<usize> {
  // we guess the order by looking at the first two
  let order = report[0].cmp(&report[1]);

  // edge case: what if we guessed the order wrong? we need to check at least the first 4.
  if report.len() >= 4 {
    let second_ord = report[1].cmp(&report[2]);
    let third_ord  = report[2].cmp(&report[3]);
    
    // one of the first two numbers are wrong; we thus cannot use them to guess the order.
    if second_ord == third_ord && order != second_ord {
      return Some(0)
    }
  }

  for (i, window) in report.windows(2).enumerate() {
    let a = window[0];
    let b = window[1];
    let curr_order = a.cmp(&b);
    let diff = a.abs_diff(b);

    if order != curr_order || !(1..=3).contains(&diff) {
      // remove the second from pair
      return Some(i)
    }
  }

  None
}