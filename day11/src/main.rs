fn main() {
  let content = std::fs::read_to_string("input.txt").expect("could not read file");
  let mut old_grid : Vec<Vec<Seat>> = content.lines().map(|l| parse_row(l)).collect();
  loop {
    //let new_grid = evolve(4, adjacent_pt1, &old_grid);
    let new_grid = evolve(5, adjacent_pt2, &old_grid);
    if new_grid.iter().flat_map(|r| r.iter()).eq(old_grid.iter().flat_map(|r| r.iter())) {
      println!("{:?}", new_grid.iter().flat_map(|r| r.iter()).filter(|&s| *s == Seat::Occupied).count());
      break;
    }
    old_grid = new_grid;
  }
}

fn parse_row(line : &str) -> Vec<Seat> {
  line.chars().map(|c| {
    match c {
      '.' => Seat::Floor,
      'L' => Seat::Empty,
      '#' => Seat::Occupied,
      _ => {
        panic!()
      }
    }
  }).collect()
}

fn evolve(
  too_full_threshold : usize,
  adjacent_fn : fn(coord : (i32,i32),  grid : &Vec<Vec<Seat>>) -> Vec<Seat>,
  grid : &Vec<Vec<Seat>>) -> Vec<Vec<Seat>> {
  let mut new_grid = Vec::new();
  for y in 0..grid.len() {
    let mut new_row = Vec::new();
    for x in 0..grid[y].len() {
      let coord = (y as i32, x as i32);
      let seat = lookup(coord, grid);
      let new_seat = match seat {
        Seat::Floor => Seat::Floor,
        Seat::Empty => {
          if adjacent_fn(coord, grid).iter().any(|&s| s == Seat::Occupied) {
            Seat::Empty
          } else {
            Seat::Occupied
          }
        },
        Seat::Occupied => {
          let occupied_count = adjacent_fn(coord, grid).iter().filter(|s| **s == Seat::Occupied).count();
          if occupied_count >= too_full_threshold {
            Seat::Empty
          } else {
            Seat::Occupied
          }
        },
        _ => panic!()
      };
      new_row.push(new_seat);
    }
    new_grid.push(new_row);
  }
  return new_grid;
}

fn adjacent_pt1(coord : (i32,i32), grid : &Vec<Vec<Seat>>) -> Vec<Seat> {
  let mut seats = Vec::new();
  let (y, x) = coord;
  for dy in -1..=1 {
    for dx in -1..=1 {
      if dy == 0 && dx == 0 {
        continue;
      }
      seats.push(lookup((y+dy, x+dx), grid));
    }
  }
  return seats;
}

fn adjacent_pt2(coord : (i32,i32), grid : &Vec<Vec<Seat>>) -> Vec<Seat> {
  let mut seats = Vec::new();
  for dy in -1..=1 {
    for dx in -1..=1 {
      if dy == 0 && dx == 0 {
        continue;
      }
      seats.push(lookup_dir(coord, (dy,dx), grid));
    }
  }
  return seats;
}

fn lookup_dir(coord : (i32,i32), dir : (i32,i32), grid : &Vec<Vec<Seat>>) -> Seat {
  let new_coord = (coord.0 + dir.0, coord.1 + dir.1);
  match lookup(new_coord, grid) {
    Seat::Floor => lookup_dir(new_coord, dir, grid),
    other => other
  }
}

fn lookup(coord : (i32,i32), grid : &Vec<Vec<Seat>>) -> Seat {
  if coord.0 < 0 || coord.0 as usize >= grid.len() {
    return Seat::None;
  }
  let row = &grid[coord.0 as usize];
  if coord.1 < 0 || coord.1 as usize >= row.len() {
    return Seat::None;
  }
  return row[coord.1 as usize];
}

#[derive(Debug,Copy,Clone,PartialEq)]
enum Seat {
  Floor,
  Empty,
  Occupied,
  None
}
