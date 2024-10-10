

fn main() {
  let input = include_str!("20.txt");
  let crypted: Vec<i64> = input.split_whitespace().map(|n| n.parse().unwrap()).collect();
  let mut decrypted = crypted.clone();

  for (i, curr) in crypted.iter().enumerate() {
    let curr_idx = decrypted.iter().position(|n| n == curr).unwrap() as i64; 
    
    decrypted.remove(curr_idx as usize);
    let new_idx = (curr_idx + curr).rem_euclid(decrypted.len() as i64);
    decrypted.insert(new_idx as usize, *curr);
  }

  let mut result1 = 0;
  let zero_idx = decrypted.iter().position(|n| *n == 0).unwrap();
  for i in 1..=3 {
    let idx = (zero_idx + i*1000) % decrypted.len();
    result1 += decrypted[idx];
  }

  // 7462 too high
  println!("{result1}");
}