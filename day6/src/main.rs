use std::collections::HashSet;

struct Group {
  combined_answers : HashSet<char>
}

fn main() {
  let content = std::fs::read_to_string("input.txt")
    .expect("could not read file");
  let groups = lines_to_groups(content.lines().collect());
  let sum_of_group_answers : usize =
    groups.iter().map(|g| g.combined_answers.len()).sum();
  println!("{}", sum_of_group_answers);
}

fn lines_to_groups(lines : Vec<&str>) -> Vec<Group> {
  let mut groups = Vec::new();
  let mut maybe_current_answers : Option<HashSet<char>> = None;
  for line in lines {
    let line = line.trim();
    if line.len() == 0 {
      if let Some(current_answers) = maybe_current_answers {
        groups.push(Group { combined_answers: current_answers.clone() });
        maybe_current_answers = None
      }
    } else {
      let person_answers : HashSet<char> = line.chars().collect();
      match maybe_current_answers {
        None => maybe_current_answers = Some(person_answers),
        Some(current_answers) => {
          maybe_current_answers = Some(
            current_answers
            .intersection(&person_answers)
            .map(|&a| a)
            .collect())
        }
      }
    }
  }
  if let Some(current_answers) = maybe_current_answers {
    groups.push(Group { combined_answers: current_answers.clone() });
  }
  return groups;
}
