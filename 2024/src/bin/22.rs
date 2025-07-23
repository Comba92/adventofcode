use std::collections::HashMap;

fn main() {
  let input = include_str!("22.txt");
  
  let nums = input.lines()
    .map(|line| line.parse::<i64>().unwrap())
    .collect::<Vec<_>>();
  
  let mut res1 = 0;
  let mut final_seqs_prices: HashMap<_, i64> = HashMap::new();
  for n in nums {
    let mut secret = n;
    let mut prev_price = n % 10;
    let mut prices = Vec::new();
    let mut diffs = Vec::new();

    // compute secrets and store prices and differences
    for _ in 0..=2000 {
      secret = compute_secret(secret);
      let price = secret % 10;
      let diff = price - prev_price;
      prev_price = price;
      prices.push(price);
      diffs.push(diff);
    }
    res1 += secret;

    // build map of diff sequences and max price 
    let mut seqs_prices = HashMap::new();
    for (i, seq) in diffs.windows(4).enumerate() {
      // WE ONLY CARE ABOUT THE FIRST OCCURENCE OF SEQ. ALL THE NEXT WILL BE DISCARDED
      seqs_prices.entry(seq).or_insert_with(|| prices[i+3]);
    }

    // add sequences to final map and sum price
    // the result will be the sequence with the highest price
    for (seq, price) in seqs_prices {
      let seq_owned: [i64; 4] = seq[0..4].try_into().unwrap();
      *final_seqs_prices.entry(seq_owned).or_default() += price;
    }
  }

  let res2 = final_seqs_prices.iter()
    .max_by_key(|e| e.1)
    .unwrap();

  println!("{res1} {res2:?}");
}

fn compute_secret(mut n: i64) -> i64 {
  let mut res = n * 64;
  n = res ^ n;
  n = n % 16777216;
  res = n / 32;
  n = res ^ n;
  n = n % 16777216;
  res = n * 2048;
  n = res ^ n;
  n = n % 16777216;
  n
}