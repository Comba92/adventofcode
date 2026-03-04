type IntervalId = u32;

#[derive(Default, Debug)]
struct IntervalNode {
    left: Option<IntervalId>,
    right: Option<IntervalId>,
    start: u64,
    end: u64,
    upper_bound: u64,
}

#[derive(Default, Debug)]
struct IntervalTree {
    nodes: Vec<IntervalNode>,
}

impl IntervalTree {
    pub fn add(&mut self, start: u64, end: u64) {
        let mut prev = None;
        let mut it = (!self.nodes.is_empty()).then_some(0);

        while let Some(id) = it {
            prev = Some(id);
            let node = &mut self.nodes[id as usize];
            if start < node.start {
                it = node.left;
            } else {
                it = node.right;
            }

            // update upper bound
            node.upper_bound = node.upper_bound.max(end);
        }

        // add links to parent
        if let Some(id) = prev {
            let new_id = self.nodes.len() as u32;
            let node = &mut self.nodes[id as usize];

            if start < node.start {
                node.left = Some(new_id);
            } else {
                node.right = Some(new_id);
            }
        }

        self.nodes.push(IntervalNode {
            left: None,
            right: None,
            upper_bound: end,
            start,
            end,
        });
    }

    pub fn contains(&self, n: u64) -> bool {
        self.contains_interval(n, n)
    }

    // https://en.wikipedia.org/wiki/Interval_tree#Augmented_tree
    pub fn contains_interval(&self, start: u64, end: u64) -> bool {
        fn contains_rec(tree: &IntervalTree, id: Option<IntervalId>, start: u64, end: u64) -> bool {
            let node = if let Some(id) = id {
                &tree.nodes[id as usize]
            } else {
                return false;
            };

            if start > node.upper_bound {
                return false;
            }

            // two intervals A and B overlap only when both
            // A.low ≤ B.high
            // and A.high ≥ B.low
            let overlaps = node.start <= end && node.end >= start;

            overlaps
                || contains_rec(tree, node.left, start, end)
                || ((start >= node.start) && contains_rec(tree, node.right, start, end))
        }

        contains_rec(self, Some(0), start, end)
    }
}

fn main() {
    let input = include_str!("05.txt");
    let input = input.replace("\n\r", "\n");

    let (ranges_str, avaibles_str) = input.split_once("\n\n").unwrap();

    let mut tree = IntervalTree::default();
    let mut intervals = Vec::new();

    for line in ranges_str.lines() {
        let (left, right) = line.split_once('-').unwrap();
        let (left, right) = (left.parse::<u64>().unwrap(), right.parse::<u64>().unwrap());

        intervals.push((left, right));
        tree.add(left, right);
    }

    let mut sum1 = 0;
    for line in avaibles_str.lines() {
        let n = line.parse::<u64>().unwrap();
        if tree.contains(n) {
            sum1 += 1;
        }
    }

    intervals.sort_by_cached_key(|e| e.0);
    let mut merged = vec![intervals[0]];

    for interval in intervals.iter().skip(1) {
        let last = merged.last_mut().unwrap();
        if last.1 < interval.0 {
            // new interval
            merged.push(*interval);
        } else {
            // there is overlap
            last.1 = last.1.max(interval.1);
        }
    }

    let mut sum2 = 0;
    for interval in merged {
        let sub = interval.1 + 1 - interval.0;
        sum2 += sub;
    }

    println!("{sum1} {sum2}");
}
