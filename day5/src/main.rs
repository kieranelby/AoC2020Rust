use std::collections::HashSet;

const NROWS: i32 = 128;
const NCOLS: i32 = 8;

fn main() {
  let content = std::fs::read_to_string("input.txt")
    .expect("could not read file");
  let listed_seat_ids : HashSet<i32> =
    content.lines()
    .map(|line| seat_spec_to_seat_id(line.trim()))
    .collect();
  let all_seat_ids : HashSet<i32> =
    (0..(NROWS*NCOLS)).collect();
  let missing_seat_ids : HashSet<i32> =
    all_seat_ids.difference(&listed_seat_ids).map(|&x| x).collect();
  let my_seat_id_candidates : HashSet<i32> =
    missing_seat_ids.iter()
      .filter(|&s| listed_seat_ids.contains(&(s - 1)) &&
                   listed_seat_ids.contains(&(s + 1)))
      .map(|&x| x).collect();
  println!("{:?}", &my_seat_id_candidates);
}

fn seat_spec_to_seat_id(s : &str) -> i32 {
  return seat_posn_to_seat_id(seat_spec_to_seat_posn(s));
}

fn seat_posn_to_seat_id(p : (i32, i32)) -> i32 {
  return p.0 * 8 + p.1;
}

fn seat_spec_to_seat_posn(s : &str) -> (i32, i32) {
  return (bsp_to_num(&s[0..7], 128), bsp_to_num(&s[7..10], 8))
}

fn bsp_to_num(s : &str, n : i32) -> i32 {
  let mut r = 0..n;
  for c in s.chars() {
    if c == 'F' || c == 'L' {
      r = r.start..(r.start + r.end)/2;
    } else {
      r = (r.start + r.end)/2..r.end;
    }
  }
  assert_eq!(r.start, r.end-1);
  return r.start;
}

#[test]
fn test_bsp_to_num() {
  assert_eq!(bsp_to_num("FBFBBFF", 128), 44);
}

#[test]
fn test_seat_spec_to_seat_posn() {
  assert_eq!(seat_spec_to_seat_posn("FBFBBFFRLR"), (44,5));
}
