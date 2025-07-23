use std::{cmp::Ordering, collections::{HashMap, HashSet}};

/*
13 -> 97, 61, 29, 47, 75, 53
29 -> 75, 97, 53, 61, 47
47 -> 97, 75
53 -> 47, 75, 61, 97
61 -> 97, 47, 75
75 -> 97

75,97,47,61,53
97,75,47,61,53

update: 75,47,61,53,29

present: 29, 47, 53, 61, 75
visited: []

75
get 75's intersection with present -> []
is a subset of visited? yes

visited: 75
47
get 47's intersection with present -> 75
is a subset of visited? yes

visited: 75, 47
61
get 61's instersection with present -> 47, 75
is a subset of visited? yes

...

update: 61,13,29

present: 13, 29, 61
visited: []

61
get 61's intersection with present -> []
is a subset of visited? yes

visited: 61
13
get 13's intersection with present -> 29
is a subset of visited? no


*/

fn main() {
  let input = include_str!("05.txt");

  let (orders, updates) = input.split_once("\n\r\n").unwrap();

  let rules_iter = orders
  .lines()
  .map(|line| line.split_once('|').unwrap())
  .map(|pair| (pair.0.parse::<i32>().unwrap(), pair.1.trim().parse::<i32>().unwrap()));

  let rules = rules_iter.clone()
  .fold(HashMap::new(), |mut map, pair| {
    map.entry(pair.1)
      .and_modify(|e: &mut HashSet<i32>| { e.insert(pair.0); })
      .or_insert(HashSet::from([pair.0]));
    
    map
  });

  // println!("Rules: {rules:#?}");

  let updates = updates
    .lines()
    .map(|line| line.split(',').map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>());

  let mut visited = HashSet::new();
  let mut res1 = 0;
  let mut res2 = 0;

  for mut update in updates {
    let present: HashSet<i32> = HashSet::from_iter(update.iter().copied());

    let mut valid = true;
    for &n in update.iter() {
      let deps = rules.get(&n);

      // we check if we have visited all its constraints
      if let Some(deps) = deps {
        let mut curr_deps = deps
          .intersection(&present);

        // if curr_deps is not subset of visited, then its not valid
        // this means not all constraints of n were visited
        if !curr_deps.all(|x| visited.contains(x)) {
          valid = false;
          break;
        }
      } else {
        // has no constraints, simply add it to visited
      }
      
      visited.insert(n);
    }

    if valid {
      let res = update[update.len()/2];
      res1 += res;
    } else {
      update.sort_by(|a, b| {
        let a_deps = rules.get(a);
        let b_deps = rules.get(b);

        match (a_deps, b_deps) {
          (None, None) => Ordering::Equal,
          (None, Some(b_deps)) => {
            if b_deps.contains(a) {
              Ordering::Less
            } else {
              Ordering::Equal
            }
          },
          (Some(a_deps), None) => {
            if a_deps.contains(b) {
              Ordering::Greater
            } else {
              Ordering::Equal
            }
          },
          (Some(a_deps), Some(b_deps)) => {
            if a_deps.contains(b) {
              Ordering::Greater
            } else if b_deps.contains(a) {
              Ordering::Less
            } else {
              Ordering::Equal
            }
          }
        }
      });

      let res = update[update.len()/2];
      res2 += res;
    }

    visited.clear();
  }

  println!("{res1} {res2}");
}