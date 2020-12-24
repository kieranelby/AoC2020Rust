use std::collections::HashSet;

fn main() {
  let content = std::fs::read_to_string("input.txt").expect("could not read file");
  //let content = std::fs::read_to_string("example.txt").expect("could not read file");
  //let content = "nwwswee";
  let instructions = read_instructions(&content);
  let mut black_tiles : HashSet<Coordinate> = HashSet::new();
  for directions in instructions {
    let mut coord : Coordinate = (0,0);
    for &direction in &directions {
      coord = move_one(coord, direction);
      println!("moved {:?} to {:?}", direction, coord);
    }
    if black_tiles.contains(&coord) {
      println!("flipped to white @ {:?}", coord);
      black_tiles.remove(&coord);
    } else {
      println!("flipped to black @ {:?}", coord);
      black_tiles.insert(coord);
    }
  }
  println!("pt1 answer = {} black tiles", black_tiles.len());
  for day in 1..=100 {
    for coord in compute_flips(&black_tiles) {
      if black_tiles.contains(&coord) {
        //println!("flipped to white @ {:?}", coord);
        black_tiles.remove(&coord);
      } else {
        //println!("flipped to black @ {:?}", coord);
        black_tiles.insert(coord);
      }
    }
    println!("Day {}: {:?}", day, black_tiles.len());
  }
}

type Coordinate = (isize,isize);

#[derive(Debug,Copy,Clone)]
enum Direction {
  East, SouthEast, SouthWest, West, NorthWest, NorthEast
}

fn move_one(coordinate : Coordinate, direction : Direction) -> Coordinate {
  let (x, y) = coordinate;
  if y % 2 == 0 {
    match direction {
      Direction::East      => (x+1, y),
      Direction::SouthEast => (x+1, y-1),
      Direction::SouthWest => (x, y-1),
      Direction::West      => (x-1, y),
      Direction::NorthWest => (x, y+1),
      Direction::NorthEast => (x+1, y+1),
    }
  } else {
    match direction {
      Direction::East      => (x+1, y),
      Direction::SouthEast => (x, y-1),
      Direction::SouthWest => (x-1, y-1),
      Direction::West      => (x-1, y),
      Direction::NorthWest => (x-1, y+1),
      Direction::NorthEast => (x, y+1),
    }
  }
}

fn read_instructions(input : &str) -> Vec<Vec<Direction>> {
  let mut instructions = Vec::new();
  for line in input.lines() {
    let directions = read_directions(line);
    instructions.push(directions);
  }
  return instructions;
}

fn read_directions(line : &str) -> Vec<Direction> {
  let mut directions = Vec::new();
  let mut tail = line;
  loop {
    if tail.starts_with("e") {
      directions.push(Direction::East);
      tail = &tail[1..];
    } else if tail.starts_with("se") {
      directions.push(Direction::SouthEast);
      tail = &tail[2..];
    } else if tail.starts_with("sw") {
      directions.push(Direction::SouthWest);
      tail = &tail[2..];
    } else if tail.starts_with("w") {
      directions.push(Direction::West);
      tail = &tail[1..];
    } else if tail.starts_with("nw") {
      directions.push(Direction::NorthWest);
      tail = &tail[2..];
    } else if tail.starts_with("ne") {
      directions.push(Direction::NorthEast);
      tail = &tail[2..];
    } else if tail.is_empty() {
      break;
    } else {
      panic!("not a direction");
    }
  }
  return directions;
}

fn compute_flips(black_tiles : &HashSet<Coordinate>) -> Vec<Coordinate> {
  if black_tiles.is_empty() {
    return Vec::new();
  }
  let lowest_x = black_tiles.iter().map(|c| c.0).min().unwrap();
  let highest_x = black_tiles.iter().map(|c| c.0).max().unwrap();
  let lowest_y = black_tiles.iter().map(|c| c.1).min().unwrap();
  let highest_y = black_tiles.iter().map(|c| c.1).max().unwrap();
  let mut flips = Vec::new();
  for y in lowest_y-2 .. highest_y+2 {
    for x in lowest_x-2 .. highest_x+2 {
      let coord = (x,y);
      let black_neighbours = count_black_neighbours(black_tiles, coord);
      if black_tiles.contains(&coord) {
        if black_neighbours == 0 || black_neighbours > 2 {
          flips.push(coord);
        }
      } else {
        if black_neighbours == 2 {
          flips.push(coord);
        }
      }
    }
  }
  flips
}

fn count_black_neighbours(black_tiles : &HashSet<Coordinate>, coord : Coordinate) -> usize {
  [ Direction::East,
    Direction::SouthEast,
    Direction::SouthWest,
    Direction::West,
    Direction::NorthWest,
    Direction::NorthEast ]
  .iter()
  .filter(|&d| black_tiles.contains(&move_one(coord, *d)))
  .count()
}
