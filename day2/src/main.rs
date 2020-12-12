use std::convert::TryInto;

fn main() {
  let content = std::fs::read_to_string("day2.input.txt")
    .expect("could not read file");
  let mut num_valid = 0;
  for line in content.lines() {
    let mut line_splitter = line.splitn(2, ':');
    let policy = line_splitter.next().unwrap();
    let password = line_splitter.next().unwrap().trim();
    let mut policy_splitter = policy.splitn(2, ' ');
    let times = policy_splitter.next().unwrap();
    let letter = policy_splitter.next().unwrap().chars().next().unwrap();
    let mut times_splitter = times.splitn(2, '-');
    let policy_num_1 : u32 = times_splitter.next().unwrap().parse().expect("bad times");
    let policy_num_2 : u32 = times_splitter.next().unwrap().parse().expect("bad times");
    if is_valid_part_two(password, letter, policy_num_1, policy_num_2) {
      num_valid = num_valid + 1;
      println!("valid line={}", line)
    } else {
      println!("INVALID line={}", line)
    }
  }
  println!("num_valid={}", num_valid)
}

fn is_valid_part_one (password : &str, letter : char, times_min : u32, times_max : u32) -> bool {
  let num_occurrences = count_occurences(letter, password);
  return num_occurrences >= times_min && num_occurrences <= times_max;
}

fn count_occurences (letter : char, word : &str) -> u32 {
  return word.chars().fold(0, |acc, c| if c == letter { acc + 1 } else { acc })
}

fn is_valid_part_two (password : &str, letter : char, pos_1 : u32, pos_2 : u32) -> bool {
  let found_1 = password.chars().nth((pos_1 - 1).try_into().unwrap()).unwrap();
  let found_2 = password.chars().nth((pos_2 - 1).try_into().unwrap()).unwrap();
  println!("{},{},{},{},{},{},{},{},{}", password, letter, pos_1, pos_2, found_1, found_2, (letter == found_1), (letter == found_2), (letter == found_1) != (letter == found_2));
  return (letter == found_1) != (letter == found_2);
}

#[test]
fn test_is_valid_part_two() {
    assert_eq!(is_valid_part_two("abcde", 'a', 1, 3), true);
    assert_eq!(is_valid_part_two("cdefg", 'b', 1, 3), false);
    assert_eq!(is_valid_part_two("ccccccccc", 'c', 2, 9), false);
    assert_eq!(is_valid_part_two("abcde", 'e', 1, 5), true);
}
