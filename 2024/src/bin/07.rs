fn main() {
  let input = include_str!("07.txt");

  let equations = input
  .lines()
  .map(|line| line.split_once(':').unwrap())
  .map(|pair| {
    let res = pair.0.parse::<u64>().unwrap();
    let nums = pair.1.split_whitespace()
      .map(|n| n.parse::<u64>().unwrap()).collect::<Vec<_>>();

    (res, nums)
  });
  
  let mut res1 = 0;
  let mut res2 = 0;
  for eq in equations {
    if find_solution1(&eq.1[1..], eq.1[0], eq.0) {
      res1 += eq.0;
    } 
    
    if find_solution2(&eq.1[1..], eq.1[0], eq.0) {
      res2 += eq.0;
    }
  }

  println!("{res1}");
  println!("{res2}");
}

fn find_solution1(nums: &[u64], curr: u64, target: u64) -> bool {
  if nums.is_empty() { return curr == target; }

  if find_solution1(&nums[1..], curr + nums[0], target) {
    return true;
  }

  if find_solution1(&nums[1..], curr * nums[0], target) {
    return true;
  }

  return false;
}

fn find_solution2(nums: &[u64], curr: u64, target: u64) -> bool {
  if nums.is_empty() { return curr == target; }

  if find_solution2(&nums[1..], curr + nums[0], target) {
    return true;
  }

  if find_solution2(&nums[1..], curr * nums[0], target) {
    return true;
  }

  let concat = format!("{}{}", curr.to_string(), nums[0].to_string())
    .parse::<u64>().unwrap();

  if find_solution2(&nums[1..], concat, target) {
    return true;
  }

  return false;
}