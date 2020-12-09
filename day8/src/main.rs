use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

#[derive(Debug)]
struct InstructionParseError;

impl FromStr for Instruction {
    type Err = InstructionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let operation_name = parts.next().ok_or(InstructionParseError)?;
        let argument_text = parts.next().ok_or(InstructionParseError)?;
        let argument: isize = argument_text.parse().map_err(|_| InstructionParseError)?;
        match &operation_name as &str {
            "acc" => Ok(Self::Acc(argument)),
            "jmp" => Ok(Self::Jmp(argument)),
            "nop" => Ok(Self::Nop(argument)),
            _ => Err(InstructionParseError),
        }
    }
}

#[derive(Debug, Clone)]
struct Device {
    code: Vec<Instruction>,
    accumulator: isize,
    program_counter: usize,
    executed_locations: HashSet<usize>,
}

fn load_program<'a>(lines: impl Iterator<Item = &'a str>) -> Device {
    Device {
        code: lines.map(|line| line.parse().unwrap()).collect(),
        accumulator: 0,
        program_counter: 0,
        executed_locations: HashSet::new(),
    }
}

#[derive(Debug, Clone, PartialEq)]
enum ExecuteOutcome {
    InfiniteLoop,
    Terminate,
    InvalidJump,
}

fn execute(initial_device: &Device) -> (ExecuteOutcome, Device) {
    //println!("executing: {:?}", initial_device);
    let mut device = initial_device.clone();
    loop {
        let pc = device.program_counter;
        if pc == device.code.len() {
            return (ExecuteOutcome::Terminate, device);
        }
        if pc > device.code.len() {
            return (ExecuteOutcome::InvalidJump, device);
        }
        if !device.executed_locations.insert(pc) {
            return (ExecuteOutcome::InfiniteLoop, device);
        }
        let instruction = &device.code[device.program_counter];
        match &instruction {
            Instruction::Acc(value) => {
                device.accumulator = device.accumulator + value;
                device.program_counter = pc + 1;
            }
            Instruction::Jmp(value) => {
                if *value < 0 && value.abs() as usize > pc {
                    return (ExecuteOutcome::InvalidJump, device);
                }
                device.program_counter = pc.wrapping_add(*value as usize)
            }
            Instruction::Nop(_) => {
                device.program_counter = pc + 1;
            }
        }
    }
}

fn mutate(device: &Device) -> Vec<Device> {
    let mut mutants = Vec::new();
    for location in 0..device.code.len() {
        let instruction = device.code[location];
        match instruction {
            Instruction::Acc(_) => continue,
            Instruction::Jmp(value) => mutants.push(with_changed_instruction(
                device,
                location,
                Instruction::Nop(value),
            )),
            Instruction::Nop(value) => mutants.push(with_changed_instruction(
                device,
                location,
                Instruction::Jmp(value),
            )),
        }
    }
    return mutants;
}

fn with_changed_instruction(device: &Device, location: usize, replacement: Instruction) -> Device {
    let mut new_device = device.clone();
    new_device.code[location] = replacement;
    return new_device;
}

fn main() {
    let content = std::fs::read_to_string("input.txt").expect("could not read file");
    let device = load_program(content.lines());
    let (outcome, final_state) = execute(&device);
    assert_eq!(outcome, ExecuteOutcome::InfiniteLoop);
    println!("PART ONE: acc={}", final_state.accumulator);
    let mutants = mutate(&device);
    let good_mutants: Vec<&Device> = mutants
        .iter()
        .filter(|&m| execute(m).0 == ExecuteOutcome::Terminate)
        .collect();
    assert_eq!(good_mutants.len(), 1);
    let (_outcome, final_state) = execute(good_mutants[0]);
    println!("PART TWO: acc={}", final_state.accumulator);
}
