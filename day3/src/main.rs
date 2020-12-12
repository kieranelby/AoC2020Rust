fn main() {
  let content = std::fs::read_to_string("day3.input.txt")
    .expect("could not read file");
  let mut rows = Vec::new();
  for line in content.lines() {
    rows.push(line.trim())
  }
  let multiplied_trees =
    count_trees(&rows, 1, 1) *
    count_trees(&rows, 3, 1) *
    count_trees(&rows, 5, 1) *
    count_trees(&rows, 7, 1) *
    count_trees(&rows, 1, 2);
  println!("{}", multiplied_trees)
}

fn count_trees(map : &Vec<&str>, dx : usize, dy : usize) -> usize {
  let height = map.len();
  let width = map.iter().nth(0).unwrap().len();
  let mut y = 0;
  let mut x = 0;
  let mut num_trees = 0;
  loop {
    let row = map.iter().nth(y).unwrap();
    let pixel = row.chars().nth(x).unwrap();
    if pixel == '#' {
      num_trees = num_trees + 1;
    }
    y = y + dy;
    x = x + dx;
    if y >= height {
      break;
    }
    while x >= width {
      x = x - width;
    }
  }
  return num_trees;
}
