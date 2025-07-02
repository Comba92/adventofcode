use std::collections::{HashMap, HashSet};

fn main() {
  let input = include_str!("23.txt");

  let mut map: HashMap<&str, HashSet<&str>> = HashMap::new();
  for line in input.lines() {
    let pair = line
      .split_once('-').unwrap();
    
    map.entry(pair.0)
      .and_modify(|e| { e.insert(pair.1); })
      .or_insert(HashSet::from([pair.1]));

    map.entry(pair.1)
      .and_modify(|e| { e.insert(pair.0); })
      .or_insert(HashSet::from([pair.0]));
  }

  println!("{map:#?}");
  let mut triples= HashSet::new();
  
  for (upper, set) in &map {
    for lower in set {
      let intersection = map[lower]
        .intersection(set);
      
      for third in intersection {
        let mut triple = [upper, lower, third];
        triple.sort();
        triples.insert(triple);
      }
    }
  }

  let res1 = triples.iter()
    .filter(|t| t.iter().any(|x| x.starts_with('t')))
    .count();

  // println!("{triples:?}");
  println!("{res1}");
}