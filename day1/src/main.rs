fn main() {
  let content = std::fs::read_to_string("day1.input.txt")
    .expect("could not read file");
  let mut nums = Vec::new();
  for line in content.lines() {
    let num : i64 = line.parse()
      .expect("line was not a number");
    nums.push(num)
  }
  for num_a in &nums {
    for num_b in &nums {
      for num_c in &nums {
        if num_a + num_b + num_c == 2020 {
          println!("{}", num_a * num_b * num_c)
        }
      }
    }
  }
}
