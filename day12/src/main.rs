use std::str::FromStr;
use std::num::ParseIntError;

fn main() {
  let content = std::fs::read_to_string("input.txt").expect("could not read file");
  let start = Position { latitude: 0, longitude: 0, heading: 90, waypoint: (1, 10) };
  let end1 = content.lines().map(|l| l.parse::<Instruction>().ok().unwrap()).fold(start, follow_pt1);
  println!("part one: {}", manhattan_dist(start, end1));
  let end2 = content.lines().map(|l| l.parse::<Instruction>().ok().unwrap()).fold(start, follow_pt2);
  println!("part two: {}", manhattan_dist(start, end2));
}

#[derive(Debug,Copy,Clone,PartialEq)]
enum Instruction {
  N(i32),
  S(i32),
  E(i32),
  W(i32),
  L(i32),
  R(i32),
  F(i32)
}

#[derive(Debug,Clone)]
enum InstructionParseError {
  BadLetter(char),
  BadValue(ParseIntError)
}

impl FromStr for Instruction {
  type Err = InstructionParseError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let value = s[1..].parse::<i32>().map_err(|e| InstructionParseError::BadValue(e))?;
    match s.chars().nth(0) {
      Some('N') => Ok(Instruction::N(value)),
      Some('S') => Ok(Instruction::S(value)),
      Some('E') => Ok(Instruction::E(value)),
      Some('W') => Ok(Instruction::W(value)),
      Some('L') => Ok(Instruction::L(value)),
      Some('R') => Ok(Instruction::R(value)),
      Some('F') => Ok(Instruction::F(value)),
      Some(other) => Err(InstructionParseError::BadLetter(other)),
      None => panic!("how did we manage to parse the value then")
    }
  }  
}

#[derive(Debug,Copy,Clone,PartialEq)]
struct Position {
  latitude : i32,
  longitude: i32,
  heading: i32,
  waypoint: (i32, i32)
}

fn manhattan_dist(a : Position, b : Position) -> i32 {
  (b.latitude - a.latitude).abs() + (b.longitude - a.longitude).abs()
}

fn follow_pt1(position : Position, instruction : Instruction) -> Position {
  match instruction {
    Instruction::N(v) => Position { latitude: position.latitude + v, ..position},
    Instruction::S(v) => follow_pt1(position, Instruction::N(-v)),
    Instruction::E(v) => Position { longitude: position.longitude + v, ..position},
    Instruction::W(v) => follow_pt1(position, Instruction::E(-v)),
    Instruction::L(v) => follow_pt1(position, Instruction::R(-v)),
    Instruction::R(v) => Position { heading: (position.heading + v).rem_euclid(360), ..position},
    Instruction::F(v) => {
      match position.heading {
        0 => follow_pt1(position, Instruction::N(v)),
        90 => follow_pt1(position, Instruction::E(v)),
        180 => follow_pt1(position, Instruction::S(v)),
        270 => follow_pt1(position, Instruction::W(v)),
        _ => panic!()
      }
    }
  }
}

fn follow_pt2(position : Position, instruction : Instruction) -> Position {
  match instruction {
    Instruction::N(v) => Position { waypoint: (position.waypoint.0 + v, position.waypoint.1), ..position},
    Instruction::S(v) => follow_pt2(position, Instruction::N(-v)),
    Instruction::E(v) => Position { waypoint: (position.waypoint.0, position.waypoint.1 + v), ..position},
    Instruction::W(v) => follow_pt2(position, Instruction::E(-v)),
    Instruction::L(v) => follow_pt2(position, Instruction::R((360 - v).rem_euclid(360))),
    Instruction::R(v) => match v {
      0 => position,
      90 => Position { waypoint: (-position.waypoint.1, position.waypoint.0) , ..position},
      180 => follow_pt2(follow_pt2(position, Instruction::R(90)), Instruction::R(90)),
      270 => follow_pt2(follow_pt2(position, Instruction::R(180)), Instruction::R(90)),
      _ => panic!()
    },    
    Instruction::F(v) => Position { 
      latitude: position.latitude + v * position.waypoint.0,
      longitude: position.longitude + v * position.waypoint.1,
      ..position
    }
  }
}
