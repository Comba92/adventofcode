use std::collections::{HashMap, HashSet};

type Vector = (i32, i32);
const LEFT: Vector = (-1, 0);
const RIGHT: Vector = (1, 0);
const UP: Vector = (0, -1);
const DOWN: Vector = (0, 1);

enum Entity { Beacon, Sensor, Empty }

fn check_and_mark(pos: Vector, marked: &mut HashSet<Vector>, sensors: &HashMap<Vector, i32>, beacons: &HashSet<Vector>) {
  if sensors.contains_key(&pos) || beacons.contains(&pos) { return }
  marked.insert(pos);
}

fn main() {
  let input = include_str!("15.txt");

  let mut sensors = HashMap::new();
  let mut beacons = HashSet::new();

  for line in input.lines() {
    let sensor: Vec<i32> = line.split(':').nth(0).unwrap()
    .chars().skip("Sensor at ".len()).collect::<String>()
    .split(',').map(|n| n.split('=').nth(1).unwrap().parse().unwrap())
    .collect();
    
    let beacon: Vec<i32> = line.split(':').nth(1).unwrap()
    .chars().skip(" closest beacon is at ".len()).collect::<String>()
    .split(',').map(|n| n.split('=').nth(1).unwrap().parse().unwrap())
    .collect();

    let sensor = (sensor[0], sensor[1]);
    let beacon = (beacon[0], beacon[1]);
    let dist = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
    sensors.insert(sensor, dist); 
    beacons.insert(beacon);
  }

  let mut marked = HashSet::new();
  for (sensor, &dist) in &sensors {
    // Longest row
    for col in (sensor.0 - dist) ..= (sensor.0 + dist) {
      check_and_mark((col, sensor.1), &mut marked, &sensors, &beacons);
    }
    
    for row in 1 ..= dist {
      for col in (sensor.0 - dist - row) ..= (sensor.0 + dist - row) {
        check_and_mark((col, sensor.1 + row), &mut marked, &sensors, &beacons);
        check_and_mark((col, sensor.1 - row), &mut marked, &sensors, &beacons);
      }
    }
  }

  println!("{}", marked.iter().filter(|p| p.1 == 10).count());
}