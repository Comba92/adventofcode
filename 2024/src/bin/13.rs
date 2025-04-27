fn main() {
  let input = include_str!("13.txt");

  let rules = input
    .split("\n\n")
    .map(|s| {
      let mut lines = s.lines();
      let a = lines.next().unwrap();
      let ax = a
        .char_indices()
        .skip_while(|(i, c)| !c.is_numeric())
        .take_while(|(i, c)| *c != ',');

      let ay = a
        .char_indices()
        .rev()
        .take_while(|(i, c)| c.is_numeric());

    })
    .collect::<Vec<_>>();
}