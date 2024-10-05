use std::{cell::RefCell, cmp::Reverse, collections::{HashMap, HashSet, VecDeque}};

#[derive(Debug)]
struct Valve {
  name: String,
  flow: u32,
  connections: Vec<String>
}

fn main() {
  let input = include_str!("16.txt");

  let mut valves = HashMap::new();

  for line in input.lines() {
    let name = line.split_whitespace().nth(1).unwrap();
    
    let flow: u32 = line.split(';').nth(0).unwrap()
    .chars().skip("Valve AA has flow ".len()).collect::<String>()
    .split('=').nth(1).unwrap().parse().unwrap();
    
    let connections: String = line.split(';').nth(1).unwrap()
    .split_whitespace().skip(4).collect();
    let connections = connections.split(',').map(|v| v.trim().to_string()).collect();

    let valve = Valve {name: name.to_string(), flow, connections};
    valves.insert(name, valve);
  }

  // // sort the connections increasing
  // for (_, valve) in valves.iter() {
  //   valve.valves.borrow_mut().sort_by_key(|v| Reverse(valves.get(v.as_str()).unwrap().flow));
  // }


  let mut current = valves.get("AA").unwrap();
  let mut opened = HashSet::new();
  
  let mut minutes: u32 = 30;
  let mut pressure = 0;
  let mut pressure_per_minute = 0;
  
  // Naive solution: sort connections descending, always visit the bigger
  // Better solution: maximize flow * minutes remaining
  // Even better? maximixe flow * minutes remaining of all connections

  while minutes > 0 {
    println!("Minute {}\tOpened: {opened:?}\nMoved to: {}\tPressure {pressure_per_minute}, Total {pressure}", 30 - minutes + 1, current.name);

    let mut best = (0, "");
    for conn in &current.connections {
      let conn = valves.get(conn.as_str()).unwrap();

      let cost: u32 = conn.connections.iter()
      .filter(|&c| !opened.contains(c))
      .map(|c| {
        valves.get(c.as_str()).unwrap().flow
      })
      .sum();

      best = best.max(((conn.flow * minutes) + cost, &conn.name));
    }

    println!("Best for {} is {}\n", current.name, best.1);

    if !opened.contains(&current.name) && current.flow * minutes >= best.0 {
      opened.insert(&current.name);

      pressure_per_minute += current.flow;
      pressure += pressure_per_minute;
      minutes -= 1;
      println!("Minute {}\tOpened: {opened:?}\n OPEN! \tPressure {pressure_per_minute}, Total {pressure}\n", 30 - minutes + 1);
    }

    if minutes == 0 { break }

    current = valves.get(best.1).unwrap();
    pressure += pressure_per_minute;
    minutes -= 1;
  }

  println!("{pressure}")
}