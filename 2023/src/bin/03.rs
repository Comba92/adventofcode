#[derive(Debug)]
struct NumberPart {
  value: u32,
  left: isize,
  right: isize,
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
    let mut tmp_str = String::new();
    let mut numbers = Vec::new();

    line.char_indices().for_each(|(i, c)| {
      if c.is_digit(10) {
        tmp_str.push(c);
      } 
      
      if (!c.is_ascii_digit() || i >= line.len()-1) && !tmp_str.is_empty() {
        let value = tmp_str.parse().unwrap();
        let right = i as isize - 1;
        let left = right - tmp_str.len() as isize + 1;

        let part = NumberPart {
          left, right, value, visited: false
        };
        numbers.push(part);
        tmp_str.clear();
      }
    });

    numbers_list.push(numbers);

    line.char_indices()
    .filter(|&(_, c)| !c.is_ascii_digit() && c != '.')
    .map(|(idx, symbol)| {
      SymbolPart {value: symbol, idx: idx as isize, row: row as isize}
    }).for_each(|part| symbols_list.push(part));
  });

  let result1 = symbols_list.iter().fold(0, |sum, symbol| {
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

  let result2 = symbols_list.iter()
  .filter(|symbol| symbol.value == '*')
  .fold(0, |sum, symbol| {    
    let mut numbers = Vec::new();
    
    for offset in [-1, 0, 1] {
      let probable_row = numbers_list.get((symbol.row + offset) as usize);
      if probable_row.is_none() { continue }
 
      probable_row.unwrap().iter()
      .filter(|num| 
        num.left - 1 <= symbol.idx && symbol.idx <= num.right + 1
      ).for_each(|num| numbers.push(num));
    }

    if numbers.len() == 2 {
      sum + numbers.iter().map(|num| num.value).product::<u32>() 
    } else { sum }
  });

  println!("{result1}{result2}");
}