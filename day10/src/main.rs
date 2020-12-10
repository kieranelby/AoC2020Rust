use std::collections::HashMap;

fn main() {
    let content = std::fs::read_to_string("input.txt").expect("could not read file");
    let mut adapters: Vec<i32> = content.lines().map(|l| l.parse().unwrap()).collect();
    adapters.sort();
    println!("adapters = {:?}", adapters);
    let mut differences = Vec::new();
    let mut current_joltage: i32 = 0;
    for adapter in &adapters {
        let difference = adapter - current_joltage;
        if difference < 1 || difference > 3 {
            panic!();
        }
        differences.push(difference);
        current_joltage = *adapter;
    }
    differences.push(3);
    println!("differences = {:?}", differences);
    println!(
        "part one = {}",
        differences.iter().filter(|&d| *d == 1).count()
            * differences.iter().filter(|&d| *d == 3).count()
    );

    let mut cache: HashMap<i32, u64> = HashMap::new();
    println!(
        "part two = {}",
        count_arrangements(0, *(adapters.last().unwrap()), &adapters, &mut cache)
    );
}

fn count_arrangements(
    current_joltage: i32,
    target_joltage: i32,
    adapters: &Vec<i32>,
    cache: &mut HashMap<i32, u64>,
) -> u64 {
    if cache.contains_key(&current_joltage) {
        return *cache.get(&current_joltage).unwrap();
    }
    let result = if current_joltage == target_joltage {
        1
    } else {
        let choices = adapters
            .iter()
            .filter(|&a| (*a - current_joltage) >= 1 && (*a - current_joltage) <= 3);
        choices
            .map(|c| count_arrangements(*c, target_joltage, adapters, cache))
            .sum()
    };
    cache.insert(current_joltage, result);
    return result;
}
