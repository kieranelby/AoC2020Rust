use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::RangeInclusive;

fn main() {
    let content = std::fs::read_to_string("input.txt").expect("could not read file");
    let notes = parse_notes(content.lines());
    //println!("{:?}", notes);
    let mut scanning_error_rate = 0;
    for nearby_ticket in &notes.nearby_tickets {
        let mut any_bad = false;
        for &value in nearby_ticket {
            if !notes
                .field_rules_by_name
                .values()
                .any(|fr| fr.is_valid(value))
            {
                scanning_error_rate = scanning_error_rate + value;
                any_bad = true;
            }
        }
    }
    println!("{:?}", scanning_error_rate);
    let mut field_position_by_field_name: HashMap<&str, usize> = HashMap::new();
    let mut positions_left : HashSet<usize> = HashSet::new();
    positions_left.extend(0..notes.your_ticket.len());
    loop {
      for field_name in notes.field_rules_by_name.keys() {
        if field_position_by_field_name.contains_key(field_name.as_str()) {
          continue;
        }
        let candidates : Vec<&usize> = positions_left.iter().filter(|&p| is_possible(&notes, field_name, *p)).collect();
        if candidates.len() == 1 {
          let position = *candidates[0];
          field_position_by_field_name.insert(field_name, position);
          positions_left.remove(&position);
        }
      }
      if positions_left.len() == 0 {
        break;
      }
    }
    println!("{:?}", field_position_by_field_name);
    let answer = field_position_by_field_name
        .iter()
        .filter(|(n, p)| n.starts_with("departure"))
        .map(|(_n, &p)| notes.your_ticket[p])
        .fold(1, |x, y| x * y);
    println!("part two = {}", answer);
}

fn is_possible (notes : &Notes, field_name : &str, position : usize) -> bool {
  let field_rules = notes.field_rules_by_name.get(field_name).unwrap();
  not_totally_invalid_nearby_tickets(notes)
      .iter()
      .all(|nt| field_rules.is_valid(nt[position]))
}

fn not_totally_invalid_nearby_tickets(notes : &Notes) -> Vec<Vec<u64>> {
  let mut not_totally_invalid_nearby_tickets = Vec::new();
  for nearby_ticket in &notes.nearby_tickets {
    let mut any_bad = false;
    for &value in nearby_ticket {
        if !notes
            .field_rules_by_name
            .values()
            .any(|fr| fr.is_valid(value))
        {
            any_bad = true;
        }
    }
    if !any_bad {
        not_totally_invalid_nearby_tickets.push(nearby_ticket.clone());
    }
  }
  return not_totally_invalid_nearby_tickets;
}

#[derive(Debug)]
struct Notes {
    ordered_field_names: Vec<String>,
    field_rules_by_name: HashMap<String, FieldRules>,
    your_ticket: Vec<u64>,
    nearby_tickets: Vec<Vec<u64>>,
}

#[derive(Debug, Clone)]
struct FieldRules {
    ranges: Vec<RangeInclusive<u64>>,
}

impl FieldRules {
    fn is_valid(&self, value: u64) -> bool {
        self.ranges.iter().any(|r| r.contains(&value))
    }
}

#[derive(Debug, Copy, Clone)]
enum ParserState {
    Rules,
    YourTicket,
    NearbyTickets,
}

fn parse_notes<'a>(lines: impl Iterator<Item = &'a str>) -> Notes {
    let mut state = ParserState::Rules;
    let mut ordered_field_names: Vec<String> = Vec::new();
    let mut field_rules_by_name: HashMap<String, FieldRules> = HashMap::new();
    let mut your_ticket: Vec<u64> = Vec::new();
    let mut nearby_tickets: Vec<Vec<u64>> = Vec::new();
    for line in lines {
        if line == "" {
            continue;
        } else if line == "your ticket:" {
            state = ParserState::YourTicket;
            continue;
        } else if line == "nearby tickets:" {
            state = ParserState::NearbyTickets;
            continue;
        }
        match state {
            ParserState::Rules => {
                let mut parts = line.split(": ");
                let field_name = parts.next().unwrap();
                let rules_str = parts.next().unwrap();
                let range_strs = rules_str.split(" or ");
                let ranges: Vec<RangeInclusive<u64>> = range_strs
                    .map(|rs| {
                        let mut bounds = rs.split('-').map(|b| b.parse::<u64>().unwrap());
                        let start = bounds.next().unwrap();
                        let end = bounds.next().unwrap();
                        start..=end
                    })
                    .collect();
                let field_rules = FieldRules { ranges: ranges };
                ordered_field_names.push(field_name.to_owned());
                field_rules_by_name.insert(field_name.to_owned(), field_rules);
            }
            ParserState::YourTicket => {
                your_ticket.extend(line.split(',').map(|s| s.parse::<u64>().unwrap()));
            }
            ParserState::NearbyTickets => {
                let ticket: Vec<u64> = line.split(',').map(|s| s.parse::<u64>().unwrap()).collect();
                nearby_tickets.push(ticket);
            }
        }
    }
    Notes {
        ordered_field_names: ordered_field_names,
        field_rules_by_name: field_rules_by_name,
        your_ticket: your_ticket,
        nearby_tickets: nearby_tickets,
    }
}
