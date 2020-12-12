#[macro_use]
extern crate lazy_static;
use std::collections::hash_map::HashMap;
use regex::Regex;

fn main() {
  let content = std::fs::read_to_string("day4.input.txt")
    .expect("could not read file");
  let num_valid =
    lines_to_passport_pairs(content.lines().collect())
    .iter()
    .map(|pp| pairs_to_fields(pp.clone()))
    .filter(|pf| has_valid_required_fields(pf.clone()))
    .count();
  println!("num_valid={}", num_valid);
}

fn lines_to_passport_pairs(lines : Vec<&str>) -> Vec<Vec<&str>> {
  let mut passports_pairs = Vec::new();
  let mut current_pairs = Vec::new();
  for line in lines {
    let line = line.trim();
    if line.len() == 0 {
      if current_pairs.len() > 0 {
        passports_pairs.push(current_pairs.clone());
        current_pairs.clear();
      }
    } else {
      let new_pairs = line.split_whitespace();
      current_pairs.extend(new_pairs);
    }
  }
  if current_pairs.len() > 0 {
    passports_pairs.push(current_pairs.clone());
    current_pairs.clear();
  }
  return passports_pairs;
}

fn pairs_to_fields(pairs : Vec<&str>) -> HashMap<&str,&str> {
  let mut fields : HashMap<&str,&str> = HashMap::new();
  for pair in pairs {
    let mut pair_splitter = pair.splitn(2, ':');
    let name = pair_splitter.next().unwrap().trim();
    let value = pair_splitter.next().unwrap().trim();
    fields.insert(name, value);
  }
  return fields;
}

fn has_valid_required_fields(fields : HashMap<&str,&str>) -> bool {
  lazy_static! {
    static ref HCL_RE: Regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
    static ref ECL_RE: Regex = Regex::new("^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    static ref PID_RE: Regex = Regex::new("^[0-9]{9}$").unwrap();
  }
  return
    fields.get("byr").and_then(|s| s.parse::<u32>().ok()).filter(|&n| n >= 1920 && n <= 2002).is_some() &&
    fields.get("iyr").and_then(|s| s.parse::<u32>().ok()).filter(|&n| n >= 2010 && n <= 2020).is_some() &&
    fields.get("eyr").and_then(|s| s.parse::<u32>().ok()).filter(|&n| n >= 2020 && n <= 2030).is_some() &&
    fields.get("hgt").filter(|s| is_valid_height(s)).is_some() &&
    fields.get("hcl").filter(|s| HCL_RE.is_match(s)).is_some() &&
    fields.get("ecl").filter(|s| ECL_RE.is_match(s)).is_some() &&
    fields.get("pid").filter(|s| PID_RE.is_match(s)).is_some();
}

fn is_valid_height(s : &str) -> bool {
  lazy_static! {
    static ref RE: Regex = Regex::new("^([0-9]+)(cm|in)$").unwrap();
  }
  match RE.captures(s) {
    None => return false,
    Some(captures) => {
      let height = (&captures[1]).parse::<u32>().unwrap();
      let unit = &captures[2];
      if unit == "cm" {
        return height >= 150 && height <= 193;
      } else {
        return height >= 59 && height <= 76;
      }
    }
  }
}
