fn main() {
  println!("example: {}", compute_encryption_key(5764801, 17807724));
  let content = std::fs::read_to_string("input.txt").expect("could not read file");
  let mut lines = content.lines();
  let card_public_key = lines.next().unwrap().parse::<u64>().unwrap();
  let door_public_key = lines.next().unwrap().parse::<u64>().unwrap();
  println!("part one: {}", compute_encryption_key(card_public_key, door_public_key));
}

fn compute_encryption_key(card_public_key : u64, door_public_key : u64) -> u64 {
  let card_loop_size = guess_loop_size(card_public_key, 7);
  let door_loop_size = guess_loop_size(door_public_key, 7);
  let encryption_key = transform(door_public_key, card_loop_size);
  let encryption_key_alt = transform(card_public_key, door_loop_size);
  assert_eq!(encryption_key, encryption_key_alt);
  encryption_key
}

fn guess_loop_size(public_key : u64, initial_subject_number : u64) -> usize {
  let mut num = 1;
  for loop_size in 0..1_000_000_000 {
    if num == public_key {
      return loop_size;
    }
    num *= initial_subject_number;
    num = num % 20201227;
  }
  panic!("not found in reasonable time");
}

fn transform(subject_number : u64, loop_size : usize) -> u64 {
  let mut num = 1;
  for _i in 0..loop_size {
    num *= subject_number;
    num = num % 20201227;
  }
  num
}
