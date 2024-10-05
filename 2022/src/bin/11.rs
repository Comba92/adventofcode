use std::cell::RefCell;

#[derive(Debug, Clone)]
struct Monkey<F1, F2> {
  items: Vec<usize>,
  inspections: usize,
  divisible_by: usize,
  op: F1,
  test: F2,
}

fn main() {
  let input = include_str!("11.txt");
  let groups = input.split("\r\n\r\n")
    .map(|group| group.lines());

  let mut monkeys = Vec::new();
  for group in  groups {
    let lines: Vec<&str> = group.skip(1).collect(); 

    let items: Vec<usize> = lines[0].split(':').nth(1).unwrap()
      .split(',').map(|item| item.trim().parse().unwrap()).collect();
    let mut ops = lines[1].split(": ").nth(1).unwrap()
      .split_whitespace().skip(2);

    let op1 = ops.next().unwrap();
    let op = ops.next().unwrap().chars().next().unwrap();
    let op2 = ops.next().unwrap();

    let is_op1_old = match op1 {
      "old" => true,
      _ => false,
    };
    let is_op2_old = match op2 {
      "old" => true,
      _ => false,
    };

    let op = move |old: usize| {
      let op1 = if is_op1_old { old } else { op1.parse().unwrap() };
      let op2 = if is_op2_old { old } else { op2.parse().unwrap() };

      match op {
        '+' => op1 + op2,
        '*' => op1 * op2,
        _ => unreachable!()
      }
    };
    
    let divisible_by: usize = lines[2].split_whitespace().last().unwrap().parse().unwrap();
    let ttrue: usize = lines[3].split_whitespace().last().unwrap().parse().unwrap();
    let tfalse: usize = lines[4].split_whitespace().last().unwrap().parse().unwrap();

    let test = move |val: usize| if val % divisible_by.clone() == 0 { ttrue } else { tfalse };
    let monkey = Monkey {items, divisible_by, op, test, inspections: 0};

    monkeys.push(RefCell::new(monkey));
  }

  let result1 = solve1(monkeys.clone());
  let result2 = solve2(monkeys.clone());

  println!("{result1} {result2}");
}

fn solve1(monkeys: Vec<RefCell<Monkey<impl Fn(usize) -> usize, impl Fn(usize) -> usize>>>) -> usize {
  for _ in 0..20 {
    for i in 0..monkeys.len() {
      let mut monkey = monkeys[i].borrow_mut();
  
      let mut count = 0;
      for &item in &monkey.items {
        count += 1;
        let new_item = (monkey.op)(item) / 3;
        let to_throw = (monkey.test)(new_item);
        monkeys[to_throw as usize].borrow_mut().items.push(new_item);
      }
  
      monkey.inspections += count;
      monkey.items.clear();
    }
  }

  let mut inspections: Vec<usize> = monkeys.iter().map(|m| m.borrow().inspections).collect();
  inspections.sort_by(|a, b| b.cmp(a));

  inspections[0] * inspections[1]
}

fn solve2(monkeys: Vec<RefCell<Monkey<impl Fn(usize) -> usize, impl Fn(usize) -> usize>>>) -> usize {
  let common_divisor = monkeys.iter().fold(1, |prod, m| prod * m.borrow().divisible_by);
  
  for _ in 0..10000 {
    for i in 0..monkeys.len() {
      let mut monkey = monkeys[i].borrow_mut();
  
      let mut count = 0;
      for &item in &monkey.items {
        count += 1;
        let new_item = (monkey.op)(item) % common_divisor;
        let to_throw = (monkey.test)(new_item);
        monkeys[to_throw as usize].borrow_mut().items.push(new_item);
      }
  
      monkey.inspections += count;
      monkey.items.clear();
    }
  }

  let mut inspections: Vec<usize> = monkeys.iter().map(|m| m.borrow().inspections).collect();
  inspections.sort_by(|a, b| b.cmp(a));

  inspections[0] * inspections[1]
}