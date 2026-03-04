fn main() {
    let input = include_str!("09.txt");

    let tiles = input
        .lines()
        .map(|line| {
            let pair = line.split_once(',').unwrap();
            (
                pair.0.parse::<u64>().unwrap(),
                pair.1.parse::<u64>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut res1 = 0;
    for t1 in &tiles {
        for t2 in &tiles {
            let area = (t1.0.abs_diff(t2.0) + 1) * (t1.1.abs_diff(t2.1) + 1);
            res1 = res1.max(area);
        }
    }

    println!("{res1}");
}
