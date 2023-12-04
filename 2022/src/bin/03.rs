use std::collections::HashSet;

fn get_priority(item: char) -> u32 {
  match item {
    'a'..='z' => item as u32 - 'a' as u32 + 1,
    'A'..='Z' => item as u32 - 'A' as u32 + 27,
    _ => unreachable!()
  }
} 

fn main() {
  let input = include_str!("03.txt");

  let result1 = input.lines().fold(0, |total_sum, line| { 
    let compartments = line.split_at(line.len() / 2);
    
    let mut compartment_maps = (HashSet::new(), HashSet::new());
    compartments.0.chars().for_each(|c| { compartment_maps.0.insert(c); });
    compartments.1.chars().for_each(|c| { compartment_maps.1.insert(c); });

    let intersection = compartment_maps.0
      .intersection(&compartment_maps.1);

    total_sum + intersection.fold(0, |sum, &item| {
      sum + get_priority(item)
    })
  });


  let mut result2 = 0;
  let mut lines = input.lines().peekable();
  while lines.peek().is_some() {
    let mut group = lines.by_ref().take(3);

    let mut comp_set_1 = HashSet::new();
    group.next().unwrap().chars().for_each(|c| { comp_set_1.insert(c); });

    let mut comp_set_2 = HashSet::new();
    group.next().unwrap().chars().for_each(|c| { comp_set_2.insert(c); });

    let mut comp_set_3 = HashSet::new();
    group.next().unwrap().chars().for_each(|c| { comp_set_3.insert(c); });

    let intersection = comp_set_1
      .intersection(&comp_set_2)
      .map(|&c| c)
      .collect::<HashSet<_>>();

    result2 += intersection.intersection(&comp_set_3).fold(0, |sum, &item| {
      sum + get_priority(item)
    })
  }

  println!("{result1} {result2}");
}