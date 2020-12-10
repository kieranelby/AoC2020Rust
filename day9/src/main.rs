fn main() {
  let content = std::fs::read_to_string("input.txt").expect("could not read file");
  let numbers : Vec<u64> = content.lines().map(|l| l.parse().unwrap()).collect();
  let preamble_size = 25;
  let first_invalid = find_first_invalid(&numbers, preamble_size);
  println!("part one = {}", first_invalid);
  let contiguous_range = find_contiguous_range(first_invalid, &numbers);
  let weakness = contiguous_range.iter().min().unwrap() + contiguous_range.iter().max().unwrap();
  println!("part two = {}", weakness);
}

fn find_first_invalid(numbers : &Vec<u64>, preamble_size : usize) -> u64 {
  let lookback_size = preamble_size;
  for i in preamble_size..numbers.len() {
    let num = numbers[i];
    let first_allowed = if i < lookback_size { 0 } else { i - lookback_size };
    if !can_make_from(num, &numbers[(first_allowed..i)]) {
      return num;
    }
  }
  panic!("ruh-roh");
}

fn can_make_from(num : u64, prev_nums : &[u64]) -> bool {
  return prev_nums.iter().any(|n1| prev_nums.iter().any(|n2| n1 + n2 == num && n1 != n2));
}

fn find_contiguous_range(num : u64, nums : &Vec<u64>) -> &[u64] {
  for i in 0..nums.len() {
    let mut attempt = 0;
    for j in (0..i).rev() {
      attempt = attempt + nums[j];
      if attempt == num {
        return &nums[j..i];
      } else if attempt > num {
        // all the numbers seem to be non-negative so no point continuing
        break;
      }
    }
  }
  panic!("oh bother");
}