use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
struct NumberPart {
  value: u32,
  left: isize,
  right: isize,
  row: usize,
  visited: bool,
}

#[derive(Debug)]
struct SymbolPart {
  row: isize,
  idx: isize,
  value: char
}

fn main() {
  let input = include_str!("03.txt");

  let mut numbers_list = Vec::new();
  let mut symbols_list = Vec::new();

  input.lines().enumerate().for_each(|(row, line)| {
    let numbers: Vec<NumberPart> = line.split(|c: char| !c.is_ascii_digit())
    .filter(|token| !token.is_empty())
    // get indexes
    .flat_map(|token| line.match_indices(token).collect::<Vec<(usize, &str)>>())
    // remove duplicates
    .collect::<HashSet<(usize, &str)>>().iter()
    .map(|&(idx, part)| {
      let left = idx as isize;
      let right = (idx + part.len() - 1) as isize;
      let value = part.parse().unwrap();
      NumberPart {left, right, value, row, visited: false}
    }).collect();

    numbers_list.push(numbers);

    line.char_indices()
    .filter(|&(_, c)| !c.is_ascii_digit() && c != '.')
    .map(|(idx, symbol)| {
      SymbolPart {value: symbol, idx: idx as isize, row: row as isize}
    }).for_each(|part| symbols_list.push(part));
  });

  let result = symbols_list.iter().fold(0, |sum, symbol| {
    let mut row_sum = 0;
    
    for offset in [-1, 0, 1] {
      let probable_row = numbers_list.get_mut((symbol.row + offset) as usize);
      if probable_row.is_none() { continue }
 
      probable_row.unwrap().iter_mut()
      .filter(|num| 
        !num.visited && num.left - 1 <= symbol.idx && symbol.idx <= num.right + 1
      ).for_each(|num| {
        num.visited = true;
        row_sum += num.value
      });
    }

    sum + row_sum
  });

  println!("{:?}", numbers_list);
  println!("{:?}", symbols_list);
  println!("{result}");
}