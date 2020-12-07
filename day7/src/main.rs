use std::collections::HashMap;

fn main() {
  let content = std::fs::read_to_string("input.txt").expect("could not read file");
  let rules_by_name: HashMap<&str, Rule> = content
      .lines()
      .map(|ln| parse_rule(ln))
      .map(|r| (r.name, r))
      .collect();
  println!(
      "part_one: {}",
      rules_by_name
          .keys()
          .filter(|rn| can_eventually_contain(&rules_by_name, rn, "shiny gold"))
          .count()
  );
  let ourselves = 1;
  println!(
      "part_two: {}",
      count_bags_in(&rules_by_name, "shiny gold") - ourselves
  );
}

struct Rule<'s> {
  name: &'s str,
  contents: Vec<(usize, &'s str)>,
}

fn parse_rule(text: &str) -> Rule {
    let mut parts = text.split(" bags contain ");
    let outer_name = parts.next().unwrap();
    let contents_text = parts.next().unwrap().trim_end_matches('.');
    return Rule {
        name: outer_name,
        contents: {
            if contents_text == "no other bags" {
                Vec::new()
            } else {
                contents_text
                    .split(", ")
                    .map(|ct| parse_contents_item(ct))
                    .collect()
            }
        },
    };
}

fn parse_contents_item(text: &str) -> (usize, &str) {
    let mut space_indices = text.match_indices(' ');
    let first_space_index = space_indices.next().unwrap().0;
    let third_space_index = space_indices.nth(1).unwrap().0;
    let quantity: usize = text[0..first_space_index].parse().unwrap();
    let inner_name = &text[first_space_index + 1..third_space_index];
    return (quantity, inner_name);
}

fn can_eventually_contain(
    rules_by_name: &HashMap<&str, Rule>,
    outer_name: &str,
    target_name: &str,
) -> bool {
    let rule = rules_by_name.get(outer_name).unwrap();
    return rule.contents.iter().any(|(_num, name)| {
        // yes, this is rather slooow
        *name == target_name || can_eventually_contain(rules_by_name, *name, target_name)
    });
}

fn count_bags_in(rules_by_name: &HashMap<&str, Rule>, outer_name: &str) -> usize {
    let rule = rules_by_name.get(outer_name).unwrap();
    let ourselves = 1;
    return rule
        .contents
        .iter()
        .map(|(num, name)| num * count_bags_in(rules_by_name, *name))
        .sum::<usize>()
        + ourselves;
}
