use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::Entry;

fn main() {
  let content = std::fs::read_to_string("input.txt").expect("could not read file");
  let foods : Vec<Food> = content.lines().map(parse_food).collect();
  println!("foods: {:?}", foods);
  let mut allegens_by_name : HashMap<String,Allegen> = HashMap::new();
  for food in &foods {
    for allegen_name in &food.allegen_names {
      let mut candidate_ingredient_names : HashSet<String> = HashSet::new();
      candidate_ingredient_names.extend(food.ingredient_names.clone());
      match allegens_by_name.entry(allegen_name.clone()) {
        Entry::Vacant(entry) => {
          entry.insert(Allegen { ingredient_name: None, candidate_ingredient_names: candidate_ingredient_names });
        },
        Entry::Occupied(entry) => {
          let mut allegen = entry.into_mut();
          allegen.candidate_ingredient_names =
            allegen.candidate_ingredient_names.intersection(&candidate_ingredient_names)
            .map(|i| i.clone()).collect();
        }
      }
    }
  }
  println!("allegens_by_name (1): {:?}", allegens_by_name);
  loop {
    let mut ingredients_made_definite = HashSet::new();
    for (_allegen_name, allegen) in &mut allegens_by_name {
      if allegen.ingredient_name.is_some() {
        continue;
      }
      if allegen.candidate_ingredient_names.len() != 1 {
        continue;
      }
      let must_be_ingredient_name = allegen.candidate_ingredient_names.iter().next().expect("len was one!");
      allegen.ingredient_name = Some(must_be_ingredient_name.clone());
      ingredients_made_definite.insert(must_be_ingredient_name.clone());
    }
    if ingredients_made_definite.len() == 0 {
      break;
    }
    for (_allegen_name, allegen) in &mut allegens_by_name {
      if allegen.ingredient_name.is_some() {
        continue;
      }
      for ingredient_name in &ingredients_made_definite {
        allegen.candidate_ingredient_names.remove(ingredient_name);
      }
    }
  }
  println!("allegens_by_name (2): {:?}", allegens_by_name);
  let known_allegen_ingredient_names : HashSet<String> =
    allegens_by_name.values()
    .filter(|a| a.ingredient_name.is_some())
    .map(|a| a.ingredient_name.as_ref().unwrap().clone())
    .collect();
  let times_safe_ingredients_appear : usize =
    foods
    .iter()
    .map(|f| {
      f.ingredient_names
      .iter()
      .filter(|&i| !known_allegen_ingredient_names.contains(i))
      .count()
    })
    .sum();
  println!("times_safe_ingredients_appear={}", times_safe_ingredients_appear);
  let mut alphabetical_allegen_names : Vec<String> = allegens_by_name.keys().map(|s| s.clone()).collect();
  alphabetical_allegen_names.sort();
  let canonical_dangerous_ingredient_list =
    alphabetical_allegen_names
    .iter()
    .map(|an| {
      allegens_by_name
      .get(an)
      .unwrap()
      .ingredient_name
      .as_ref()
      .expect("could not determine ingredient for allegen")
      .clone()
    })
    .collect::<Vec<String>>()
    .join(",");
  println!("canonical_dangerous_ingredient_list={}", canonical_dangerous_ingredient_list);
}

#[derive(Debug,Clone)]
struct Food {
  ingredient_names: Vec<String>,
  allegen_names: Vec<String>,
}

#[derive(Debug,Clone)]
struct Allegen {
  ingredient_name: Option<String>,
  candidate_ingredient_names: HashSet<String>,
}

fn parse_food(s : &str) -> Food {
  let mut parts = s.split(" (contains ");
  let ingredients_part = parts.next().expect("missing ingredients");
  let allegens_part = parts.next().expect("missing allegens").trim_end_matches(')');
  let ingredients = ingredients_part.split(" ").map(|s| s.to_string()).collect();
  let allegens = allegens_part.split(", ").map(|s| s.to_string()).collect();
  Food {
    ingredient_names: ingredients,
    allegen_names: allegens,
  }
}
