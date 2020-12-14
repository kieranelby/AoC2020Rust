use std::collections::HashMap;
use std::str::FromStr;

fn main() {
  pt1();
  pt2();
}

fn pt1() {
  let content = std::fs::read_to_string("input.txt").expect("could not read file");
  let instructions : Vec<Instruction> = content.lines().map(|l| l.parse::<Instruction>().unwrap()).collect();
  println!("instructions: {:?}", instructions);
  let mut state = MachineState::new();
  for &instruction in &instructions {
    apply_pt1(&mut state, instruction);
  }
  println!("part one state after instructions: {:?}", state);
  println!("part one: {:?}", state.memory.values().sum::<u64>());
}

fn pt2() {
  let content = std::fs::read_to_string("input.txt").expect("could not read file");
  let instructions : Vec<Instruction> = content.lines().map(|l| l.parse::<Instruction>().unwrap()).collect();
  println!("instructions: {:?}", instructions);
  let mut state = MachineState::new();
  for &instruction in &instructions {
    apply_pt2(&mut state, instruction);
  }
  println!("part two state after instructions: {:?}", state);
  println!("part two: {:?}", state.memory.values().sum::<u64>());
}

#[derive(Debug)]
struct MachineState {
  or_mask: u64,
  and_mask: u64,
  float_mask: u64,
  memory : HashMap<u64,u64>
}

impl MachineState {
  pub fn new() -> MachineState {
    let memory : HashMap<u64,u64> = HashMap::new();
    MachineState { or_mask: 0, and_mask: u64::MAX, float_mask: 0, memory: memory }
  }
}

#[derive(Debug,Copy,Clone,PartialEq)]
enum Instruction {
  SetMask(u64, u64, u64),
  Write(u64, u64)
}

fn apply_pt1(state : &mut MachineState, instr : Instruction) -> &mut MachineState {
  match instr {
    Instruction::SetMask(or_mask, and_mask, float_mask) => {
      state.or_mask = or_mask;
      state.and_mask = and_mask;
      state.float_mask = float_mask;
    },
    Instruction::Write(addr, value) => {
      let masked_value = (value | state.or_mask) & state.and_mask;
      if masked_value != 0 {
        state.memory.insert(addr, masked_value);
      } else {
        state.memory.remove(&addr);
      }
    }
  }
  return state;
}

fn apply_pt2(state : &mut MachineState, instr : Instruction) -> &mut MachineState {
  match instr {
    Instruction::SetMask(or_mask, and_mask, float_mask) => {
      state.or_mask = or_mask;
      state.and_mask = and_mask;
      state.float_mask = float_mask;
    },
    Instruction::Write(addr, value) => {
      let masked_addr = addr | state.or_mask;
      write(state, masked_addr, value, state.float_mask, 0);
    }
  }
  return state;
}

fn write(state : &mut MachineState, addr : u64, value : u64, float_mask: u64, float_bit_pos: u8) {
  if float_bit_pos == 36 {
    return;
  }
  let float_mask_mask = 1u64 << float_bit_pos;
  let our_float_mask = float_mask & float_mask_mask;
  if our_float_mask == 0 {
    state.memory.insert(addr, value);
    write(state, addr, value, float_mask, float_bit_pos + 1);
  } else {
    let zeroed_addr = addr & (!our_float_mask);
    state.memory.insert(zeroed_addr, value);
    write(state, zeroed_addr, value, float_mask, float_bit_pos + 1);
    let oned_addr = addr | our_float_mask;
    state.memory.insert(oned_addr, value);
    write(state, oned_addr, value, float_mask, float_bit_pos + 1);
  }
}

#[derive(Debug,Clone)]
enum InstructionParseError {
  BadMask,
  BadWriteAddr,
  BadWriteValue,
  UnknownInstruction
}

impl FromStr for Instruction {
  type Err = InstructionParseError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if s.starts_with("mask = ") {
      let mask = s[7..].to_string();
      let or_mask = u64::from_str_radix(&mask.replace("X", "0"), 2).map_err(|_e| InstructionParseError::BadMask)?;
      let and_mask = u64::from_str_radix(&mask.replace("X", "1"), 2).map_err(|_e| InstructionParseError::BadMask)?;
      let float_mask = u64::from_str_radix(&mask.replace("1","0").replace("X", "1"), 2).map_err(|_e| InstructionParseError::BadMask)?;
      Ok(Instruction::SetMask(or_mask, and_mask, float_mask))
    } else if s.starts_with("mem[") {
      let mut parts = s[4..].split("] = ");
      let addr = parts.next().unwrap().parse::<u64>().map_err(|_e| InstructionParseError::BadWriteAddr)?;
      let value = parts.next().unwrap().parse::<u64>().map_err(|_e| InstructionParseError::BadWriteValue)?;
      Ok(Instruction::Write(addr, value))
    } else {
      Err(InstructionParseError::UnknownInstruction)
    }
  }  
}
