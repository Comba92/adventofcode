use std::collections::{HashMap, HashSet};

fn main() {
  let input = include_str!("23.txt");

  let mut links: HashMap<&str, HashSet<&str>> = HashMap::new();
  for line in input.lines() {
    let pair = line
      .split_once('-').unwrap();
    
    links.entry(pair.0)
      .and_modify(|e| { e.insert(pair.1); })
      .or_insert(HashSet::from([pair.1]));

    links.entry(pair.1)
      .and_modify(|e| { e.insert(pair.0); })
      .or_insert(HashSet::from([pair.0]));
  }

  // println!("{map:#?}");
  let mut triples= HashSet::new();
  
  for (upper, set) in &links {
    for lower in set {
      let intersection = links[lower]
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

  println!("{res1}");
    
  
  let max_degree = links.values()
    .map(|v| v.len())
    .max().unwrap() + 1;
  // add 1 as the key is not included in set

  for clique_nodes in (0..=max_degree).rev() {
    if let Some(res2) = find_k_clique(&links, clique_nodes) {
      println!("{res2}");
      break;
    }
  }
}

fn find_k_clique(links: &HashMap<&str, HashSet<&str>>, clique_nodes: usize) -> Option<String> {
  for (upper, set) in links {
    for lower in set {
      let intersection = links[lower]
        .intersection(set);

      // intersection excludes upper and lower
      let subgraph_nodes = 2 + intersection.clone().count();
      if subgraph_nodes != clique_nodes { continue; }
    
      let mut is_clique = true;
      for &v in intersection.clone() {
        for &w in intersection.clone() {
          if v == w { continue; }
          if !links[v].contains(w) {
            is_clique = false;
            break;
          }
        }
      }

      if is_clique {
        let mut clique = intersection
          .map(|s| *s)
          .collect::<Vec<_>>();

        clique.push(upper);
        clique.push(lower);
        clique.sort();

        let res = clique.join(",");
        return Some(res);
      }
    }
  }

  return None;
}