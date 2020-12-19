use std::collections::HashMap;
use std::str::FromStr;

fn main() {
  let content = std::fs::read_to_string("input.txt").expect("could not read file");
  let (rules, messages) = parse_input(&content);
  let rule_zero = &rules.get(&0).unwrap();
  {
    println!("pt1: {}", messages.iter().filter(|m| rule_zero.is_complete_match(&rules, m)).count());
  }
  let mut rules2 = rules.clone();
  rules2.insert(8, "42 | 42 8".parse::<Rule>().unwrap());
  rules2.insert(11, "42 31 | 42 11 31".parse::<Rule>().unwrap());
  println!("pt2: {}", messages.iter().filter(|m| rule_zero.is_complete_match2(&rules2, m)).count());
}

type RuleNum = usize;
type Rules = HashMap<RuleNum,Rule>;
type Messages = Vec<String>;

#[derive(Debug,Clone)]
enum Rule {
  SingleChar(char),
  Seq(Vec<RuleNum>),
  AtLeastOne(Vec<Rule>)
}

impl Rule {

  fn is_complete_match(self : &Self, rules : &Rules, s : &str) -> bool {
    match self.check_match(rules, s) {
      None => false,
      Some(remaining) => remaining.len() == 0
    }
  }

  fn check_match<'a>(self : &Self, rules : &Rules, s : &'a str) -> Option<&'a str> {
    match self {
      Rule::SingleChar(c) => {
        if s.starts_with(*c) {
          Some(&s[1..])
        } else {
          None
        }
      },
      Rule::Seq(rule_nums) => {
        let mut p = s;
        for rule_num in rule_nums {
          let rule = rules.get(rule_num).unwrap();
          match rule.check_match(rules, p) {
            None => { return None; },
            Some(tail) => { p = tail }
          }
        }
        Some(p)
      }
      Rule::AtLeastOne(sub_rules) => {
        let mut matching_sub_rules =
          sub_rules
          .iter()
          .map(|sr| sr.check_match(rules, s))
          .filter(|cm| cm.is_some());
        match matching_sub_rules.next() {
          None => None,
          Some(known_match) => known_match
        }
      }
    }
  }

  fn is_complete_match2(self : &Self, rules : &Rules, s : &str) -> bool {
    let matches = self.check_match2(rules, s);
    matches.iter().any(|r| r.len() == 0)
  }

  fn check_match2<'a>(self : &Self, rules : &Rules, s : &'a str) -> Vec<&'a str> {
    let mut matches = Vec::new();
    match self {
      Rule::SingleChar(c) => {
        if s.starts_with(*c) {
          matches.push(&s[1..]);
        }
      },
      Rule::Seq(rule_nums) => {
        matches.push(s);
        for rule_num in rule_nums {
          let rule = rules.get(rule_num).unwrap();
          let mut next_matches = Vec::new();
          for &w in &matches {
            next_matches.extend(rule.check_match2(rules, w));
          }
          matches = next_matches;
        }
      }
      Rule::AtLeastOne(sub_rules) => {
        matches.extend(
          sub_rules
          .iter()
          .map(|sr| sr.check_match2(rules, s))
          .flatten());
      }
    }
    return matches;
  }
}

fn parse_input(s: &str) -> (Rules, Messages) {
  let mut rules = Rules::new();
  let mut messages = Messages::new();
  let mut finished_rules = false;
  for line in s.lines() {
    if line.len() == 0 {
      finished_rules = true;
      continue;
    }
    if !finished_rules {
      let mut parts = line.split(": ");
      let rule_num = parts.next().unwrap().parse::<RuleNum>().unwrap();
      let rule = parts.next().unwrap().parse::<Rule>().unwrap();
      rules.insert(rule_num, rule);
    } else {
      messages.push(line.to_owned());
    }
  }
  return (rules, messages);
}

#[derive(Debug)]
struct RuleParseError;

impl FromStr for Rule {
    type Err = RuleParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('"') {
          if s.len() != 3 || !s.ends_with('"') {
            panic!("bad single char rule {}", s);
          }
          Ok(Rule::SingleChar(s.chars().nth(1).unwrap()))
        } else if s.contains('|') {
          Ok(Rule::AtLeastOne(
            s
            .split(" | ")
            .map(|p| p.parse::<Rule>().unwrap())
            .collect()))
        } else {
          Ok(Rule::Seq(
              s
              .split(' ')
              .map(|p| p.parse::<RuleNum>().unwrap())
              .collect()))
        }
    }
}
