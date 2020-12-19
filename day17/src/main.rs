use std::collections::HashMap;

fn main() {
  pt1();
  pt2();
}

// yeah this is ugly
fn pt1() {
  let content = std::fs::read_to_string("input.txt").expect("could not read file");
  let mut pocket_dimension : HashMap<(isize,isize,isize),State> = HashMap::new();
  let mut y = 0;
  for line in content.lines() {
    let mut x = 0;
    for c in line.chars() {
      let state = match c {
        '.' => State::Inactive,
        '#' => State::Active,
        _ => panic!()
      };
      if state != State::Inactive {
        pocket_dimension.insert((x,y,0), state);
      }
      x = x + 1;
    }
    y = y + 1;
  }
  for cycle in 1..=6 {
    //println!("pd={:?}", pocket_dimension);
    let mut next_pocket_dimension : HashMap<(isize,isize,isize),State> = HashMap::new();
    let x_range = (pocket_dimension.keys().map(|(x,_y,_z)| *x).min().unwrap() - 1) ..= (pocket_dimension.keys().map(|(x,_y,_z)| *x).max().unwrap() + 1);
    let y_range = (pocket_dimension.keys().map(|(_x,y,_z)| *y).min().unwrap() - 1) ..= (pocket_dimension.keys().map(|(_x,y,_z)| *y).max().unwrap() + 1);
    let z_range = (pocket_dimension.keys().map(|(_x,_y,z)| *z).min().unwrap() - 1) ..= (pocket_dimension.keys().map(|(_x,_y,z)| *z).max().unwrap() + 1);
    //println!("bounds {:?},{:?},{:?}", x_range, y_range, z_range);
    for x in x_range.clone() {
      for y in y_range.clone() {
        for z in z_range.clone() {
          let coord = (x, y, z);
          let mut active_neighbour_count = 0;
          for dx in -1 ..= 1 {
            for dy in -1 ..= 1 {
              for dz in -1 ..= 1 {
                let neighbour_coord = (x + dx, y + dy, z + dz);
                if neighbour_coord != coord {
                  if *pocket_dimension.get(&neighbour_coord).or(Some(&State::Inactive)).unwrap() == State::Active {
                    active_neighbour_count = active_neighbour_count + 1;
                  }
                }
              }
            }
          }
          let prev_state = *pocket_dimension.get(&coord).or(Some(&State::Inactive)).unwrap();
          let new_state = if prev_state == State::Inactive {
            if active_neighbour_count == 3 {
              State::Active
            } else {
              State::Inactive
            }
          } else {
            if (2..=3).contains(&active_neighbour_count) {
              State::Active
            } else {
              State::Inactive
            }
          };
          //println!("debug {:?},{:?},{},{:?}", coord, prev_state, active_neighbour_count, new_state);
          if new_state != State::Inactive {
            next_pocket_dimension.insert(coord, new_state);
          }
        }
      }
    }
    pocket_dimension = next_pocket_dimension;
  }
  println!("pt1={}", pocket_dimension.len());
}

// yeah this is very ugly
fn pt2() {
  let content = std::fs::read_to_string("input.txt").expect("could not read file");
  let mut pocket_dimension : HashMap<(isize,isize,isize,isize),State> = HashMap::new();
  let mut y = 0;
  for line in content.lines() {
    let mut x = 0;
    for c in line.chars() {
      let state = match c {
        '.' => State::Inactive,
        '#' => State::Active,
        _ => panic!()
      };
      if state != State::Inactive {
        pocket_dimension.insert((x,y,0,0), state);
      }
      x = x + 1;
    }
    y = y + 1;
  }
  for cycle in 1..=6 {
    //println!("pd={:?}", pocket_dimension);
    let mut next_pocket_dimension : HashMap<(isize,isize,isize,isize),State> = HashMap::new();
    let x_range = (pocket_dimension.keys().map(|(x,_y,_z,_w)| *x).min().unwrap() - 1) ..= (pocket_dimension.keys().map(|(x,_y,_z,_w)| *x).max().unwrap() + 1);
    let y_range = (pocket_dimension.keys().map(|(_x,y,_z,_w)| *y).min().unwrap() - 1) ..= (pocket_dimension.keys().map(|(_x,y,_z,_w)| *y).max().unwrap() + 1);
    let z_range = (pocket_dimension.keys().map(|(_x,_y,z,_w)| *z).min().unwrap() - 1) ..= (pocket_dimension.keys().map(|(_x,_y,z,_w)| *z).max().unwrap() + 1);
    let w_range = (pocket_dimension.keys().map(|(_x,_y,_z,w)| *w).min().unwrap() - 1) ..= (pocket_dimension.keys().map(|(_x,_y,_z,w)| *w).max().unwrap() + 1);
    //println!("bounds {:?},{:?},{:?},{:?}", x_range, y_range, z_range, w_range);
    for x in x_range.clone() {
      for y in y_range.clone() {
        for z in z_range.clone() {
          for w in w_range.clone() {
            let coord = (x, y, z, w);
            let mut active_neighbour_count = 0;
            for dx in -1 ..= 1 {
              for dy in -1 ..= 1 {
                for dz in -1 ..= 1 {
                  for dw in -1 ..= 1 {
                    let neighbour_coord = (x + dx, y + dy, z + dz, w + dw);
                    if neighbour_coord != coord {
                      if *pocket_dimension.get(&neighbour_coord).or(Some(&State::Inactive)).unwrap() == State::Active {
                        active_neighbour_count = active_neighbour_count + 1;
                      }
                    }
                  }
                }
              }
            }
            let prev_state = *pocket_dimension.get(&coord).or(Some(&State::Inactive)).unwrap();
            let new_state = if prev_state == State::Inactive {
              if active_neighbour_count == 3 {
                State::Active
              } else {
                State::Inactive
              }
            } else {
              if (2..=3).contains(&active_neighbour_count) {
                State::Active
              } else {
                State::Inactive
              }
            };
            //println!("debug {:?},{:?},{},{:?}", coord, prev_state, active_neighbour_count, new_state);
            if new_state != State::Inactive {
              next_pocket_dimension.insert(coord, new_state);
            }
          }
        }
      }
    }
    pocket_dimension = next_pocket_dimension;
  }
  println!("pt2={}", pocket_dimension.len());
}

#[derive(Debug,Copy,Clone,PartialEq)]
enum State {
  Inactive, Active
}
