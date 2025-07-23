use std::{collections::{HashMap, HashSet}, ops::Shl};

fn main() {
  let input = include_str!("24.txt");
  let (wires_str, gates_str) = input.split_once("\n\r").unwrap();

  let mut wires = HashMap::new();
  let mut to_visit = HashSet::new();
  
  for wire in wires_str.lines() {
    let (name, val) = wire.split_once(": ").unwrap();
    println!("{name} {val}");
    wires.insert(name, val.trim().parse::<u8>().unwrap() == 1);
    to_visit.insert(name);
  }
  
  let gates_count = to_visit.len() + gates_str.lines().count()-1;

  loop {
    for gate in gates_str.lines().skip(1) {
      let mut tokens = gate.split_whitespace();
      let left = tokens.next().unwrap();
      let op = tokens.next().unwrap();
      let right = tokens.next().unwrap();
  
      // skip ->
      tokens.next();
  
      let dst = tokens.next().unwrap();
  
      let mut skip = false;
      if !wires.contains_key(left) {
        // to_visit.insert(left);
        skip = true;
      }
      
      if !wires.contains_key(right) {
        // to_visit.insert(right);
        skip = true;
      }

      if skip { continue; }
  
      to_visit.insert(left);
      to_visit.insert(right);
      let left = wires[left];
      let right = wires[right];
  
      let res = match op {
        "AND" => left && right,
        "OR" => left || right,
        "XOR" => left ^ right,
        _ => unreachable!("invalid logic operation")
      };
  
      wires.insert(dst, res);
      to_visit.insert(dst);
    }

    if to_visit.len() >= gates_count {
      break;
    }
  }

  let mut z_wires = wires.into_iter()
    .filter(|e| e.0.starts_with("z"))
    .collect::<Vec<_>>();

  z_wires.sort();
  let res1 = z_wires.iter().rev()
    .fold(0u64, |acc, (_, b)| acc.shl(1) | (*b as u64));

  println!("{res1}");
}