fn main() {
  pt1();
  pt2();
}

fn pt1() {
  let content = std::fs::read_to_string("input.txt").expect("could not read file");
  let mut lines = content.lines();
  let earliest_estimate = lines.next().unwrap().parse::<i64>().unwrap();
  let raw_bus_ids = lines.next().unwrap().split(',');
  let num_bus_ids : Vec<i64> =
    raw_bus_ids
      .filter(|b| *b != "x").map(|b| b.parse::<i64>().unwrap())
      .collect();
  let (earliest_time, earliest_id) = compute_earliest_bus(earliest_estimate, &num_bus_ids);
  println!("pt one: {}", earliest_id * (earliest_time - earliest_estimate));
}

fn compute_earliest_bus(min : i64, num_bus_ids : &Vec<i64>) -> (i64,i64) {
  for t in min..i64::MAX {
    for &id in num_bus_ids {
      if t % id == 0 {
        return (t, id)
      }
    }
  }
  panic!()
}

fn pt2() {
  let content = std::fs::read_to_string("input.txt").expect("could not read file");
  let mut lines = content.lines();
  let _junk = lines.next().unwrap().parse::<i64>().unwrap();
  let raw_bus_ids = lines.next().unwrap().split(',');
  let mut t = 1i64;
  let mut step = 1i64;
  let mut offset = 0i64;
  for raw_bus_id in raw_bus_ids {
    if raw_bus_id != "x" {
      let id = raw_bus_id.parse::<i64>().unwrap();
      println!("id={} offset={} step={}", id, offset, step);
      loop {
        if (t + offset) % id == 0 {
          println!("{}+{}%{}==0", t, offset, id);
          break;
        }
        t = t + step;
      }
      step = step * id;
    }
    offset = offset + 1;
  }
  println!("pt two: {}", t);
}
