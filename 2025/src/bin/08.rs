use std::{
    cmp,
    collections::{BinaryHeap, HashMap, HashSet},
    mem,
};

type Coord = (i64, i64, i64);

#[derive(PartialEq, Eq, Debug)]
struct Distance {
    dist: u64,
    indices: (usize, usize),
    // points: (Coord, Coord),
}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.dist.cmp(&other.dist)
    }
}

impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = include_str!("08.txt");

    let points = input
        .lines()
        .map(|line| {
            let mut nums = line.split(',').map(|x| x.parse::<i64>().unwrap());
            (
                nums.next().unwrap(),
                nums.next().unwrap(),
                nums.next().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    // (distance, (point_a_id, point_b_id))
    let mut distances = BinaryHeap::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let dist = euclid_dist(&points[i], &points[j]);
            let entry = Distance {
                dist,
                indices: (i, j),
                // points: (points[i], points[j]),
            };

            // max heap
            distances.push(entry);
        }
    }

    // this gets order from min to max
    let distances = distances.into_sorted_vec();

    let mut circuits = (0..points.len())
        .map(|i| HashSet::from([i]))
        .collect::<Vec<_>>();

    // point_id to circuit_id
    let mut graph = (0..points.len()).map(|x| (x, x)).collect::<HashMap<_, _>>();

    for dist in distances.iter().take(1000) {
        connect(dist, &mut graph, &mut circuits);
    }

    let mut lengths = circuits.iter().map(|x| x.len()).collect::<Vec<_>>();
    lengths.sort();

    let res1 = lengths.iter().rev().take(3).product::<usize>();
    println!("{res1}");

    for dist in distances.iter().skip(1000) {
        connect(dist, &mut graph, &mut circuits);

        // if they are all the same...
        if graph.values().min() == graph.values().max() {
            let p1 = &points[dist.indices.0];
            let p2 = &points[dist.indices.1];

            let res2 = p1.0 * p2.0;
            println!("{res2}");
            break;
        }
    }
}

fn connect(dist: &Distance, graph: &mut HashMap<usize, usize>, circuits: &mut Vec<HashSet<usize>>) {
    // println!("Processing: {:?}", dist.points);

    let circ_a = *graph.get(&dist.indices.0).unwrap();
    let circ_b = *graph.get(&dist.indices.1).unwrap();

    if circ_a != circ_b {
        let mut union = mem::take(&mut circuits[circ_a]);
        union.extend(circuits[circ_b].iter());

        for junc in &union {
            graph.insert(*junc, circ_a);
        }

        circuits[circ_a] = union;
        circuits[circ_b].clear();
    }
}

fn euclid_dist(a: &Coord, b: &Coord) -> u64 {
    let x = a.0 - b.0;
    let y = a.1 - b.1;
    let z = a.2 - b.2;
    let sum = x.pow(2) + y.pow(2) + z.pow(2);
    sum.isqrt() as u64
}
