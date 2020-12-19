use std::collections::HashMap;

fn main() {
  let content = "0,13,1,16,6,17";
  let starting_numbers : Vec<usize> = content.split(',').map(|s| s.parse::<usize>().unwrap()).collect();
  let mut last_seen = HashMap::new();
  let stop_turn = 30000000;
  let mut last_num = 0;
  for turn in 1..=stop_turn {
    let num = if turn <= starting_numbers.len() {
      starting_numbers[turn - 1]
    } else {
      match last_seen.get(&last_num) {
        Some(&turn_spoken) => {
          //println!("last_num {} seen on {}", last_num, turn_spoken);
          turn - 1 - turn_spoken
        },
        None => 0
      }
    };
    last_seen.insert(last_num, turn - 1);
    last_num = num;
    //println!("{} {}", turn, num);
  }
  println!("{}", last_num);
}
