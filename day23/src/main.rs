use std::fmt;
use std::str::FromStr;
use std::num::ParseIntError;

fn main() {
  let input = "389125467";
  //let input = "476138259";
  //let num_moves = 10;
  let num_moves = 10_000_000;
  //let num_moves = 1000;
  let mut cups : Cups = input.parse().expect("input should be cup labels");
  //cups = cups.pad(100);
  cups = cups.pad(1_000_000);
  for move_num in 1..=num_moves {
    println!("-- move {} --", move_num);
    cups.pick_up_three();
    let destination = cups.choose_destination();
    println!("destination: {}", destination);
    cups.place(destination);
    cups.select_new_current_cup();
  }
  println!("-- final --");
  println!("{}", cups);
  if cups.len() <= 10 {
    println!("labels after: {}", cups.labels_after(1));
  }
  let c1 = cups.clockwise_of(1);
  let c2 = cups.clockwise_of(c1);
  println!("{} * {} + {}", c1, c2, c1 * c2);
}

type CupLabel = u32;

#[derive(Debug, Clone)]
struct Cups {
  current_cup_label: CupLabel,
  clockwise_cup_labels: Vec<CupLabel>,
  picked_up: Vec<CupLabel>,
  lowest_label: CupLabel,
  highest_label: CupLabel
}

impl Cups {

  fn new(clockwise_cup_labels: Vec<CupLabel>) -> Cups {
    let lowest_label = *clockwise_cup_labels.iter().min().expect("should be some cups");
    let highest_label = *clockwise_cup_labels.iter().max().expect("should be some cups");
    Cups {
      current_cup_label: clockwise_cup_labels[0],
      clockwise_cup_labels: clockwise_cup_labels,
      picked_up: Vec::new(),
      lowest_label: lowest_label,
      highest_label: highest_label,
    }    
  }

  fn position_of(self : &Self, cup_label: CupLabel) -> usize {
    self.clockwise_cup_labels
    .iter()
    .position(|&c| c == cup_label)
    .expect("cup must exist")
  }
  
  fn len(self : &Self) -> usize {
    self.clockwise_cup_labels.len()
  }

  fn clockwise_of(self : &Self, cup_label: CupLabel) -> CupLabel {
    let position = self.position_of(cup_label);
    let next_cup_index = if position < self.len() - 1 {
      position + 1
    } else {
      0
    };
    self.clockwise_cup_labels[next_cup_index]
  }

  fn pick_up_one(self : &mut Self) {
    let cup = self.clockwise_of(self.current_cup_label);
    self.picked_up.push(cup);
    // TODO - this is also slow
    let clockwise_cup_labels : Vec<CupLabel> =
      self.clockwise_cup_labels
      .iter()
      .filter(|&c| *c != cup)
      .map(|c| *c)
      .collect();
    self.clockwise_cup_labels = clockwise_cup_labels;
  }

  fn pick_up_three(self : &mut Self) {
    self.pick_up_one();
    self.pick_up_one();
    self.pick_up_one();
  }

  fn decrement_label(self : &Self, cup_label: CupLabel) -> CupLabel {
    if cup_label == self.lowest_label {
      self.highest_label
    } else {
      cup_label - 1
    }
  }

  fn choose_destination(self : &Self) -> CupLabel {
    let mut destination = self.current_cup_label;
    loop {
      destination = self.decrement_label(destination);
      if !self.picked_up.contains(&destination) {
        return destination;
      }
    }
  }

  fn place(self : &mut Self, destination : CupLabel) {
    let position = self.position_of(destination);
    // TODO - THIS IS SLOOOOOOWWWWW
    let mut clockwise_cup_labels = Vec::new();
    clockwise_cup_labels.extend(&self.clockwise_cup_labels[0..=position]);
    clockwise_cup_labels.extend(&self.picked_up);
    clockwise_cup_labels.extend(&self.clockwise_cup_labels[position+1..]);
    self.clockwise_cup_labels = clockwise_cup_labels;
    self.picked_up = Vec::new();
  }

  fn select_new_current_cup(self : &mut Self) {
    self.current_cup_label = self.clockwise_of(self.current_cup_label);
  }

  fn labels_after(self : &Self, cup_label : CupLabel) -> String {
    let position = self.position_of(cup_label);
    let mut s = String::new();
    for cup in &self.clockwise_cup_labels[position+1..] {
      s.push_str(&cup.to_string());
    }
    for cup in &self.clockwise_cup_labels[0..position] {
      s.push_str(&cup.to_string());
    }
    s
  }

  fn pad(self : &Self, last_cup : CupLabel) -> Cups {
    let highest_label = *self.clockwise_cup_labels.iter().max().expect("should be some cups");
    let mut clockwise_cup_labels = self.clockwise_cup_labels.clone();
    clockwise_cup_labels.extend(highest_label+1..=last_cup);
    return Cups {
      clockwise_cup_labels: clockwise_cup_labels,
      ..self.clone()
    }
  }

}

impl FromStr for Cups {
  type Err = ParseIntError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let clockwise_cup_labels =
      s
      .chars()
      .map(|c| c.to_string().parse::<CupLabel>().expect("cup labels should be small non-negative numbers"))
      .collect::<Vec<CupLabel>>();
    Ok(Cups::new(clockwise_cup_labels))
  }
}

impl fmt::Display for Cups {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "cups:")?;
    for c in &self.clockwise_cup_labels {
      if *c == self.current_cup_label {
        write!(f, " ({})", c)?;
      } else {
        write!(f, " {}", c)?;
      }
    }
    if !self.picked_up.is_empty() {
      write!(f, "; pick up:")?;
      for c in &self.picked_up {
        write!(f, " {}", c)?;
      }
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
      let input = "389125467";
      let mut cups : Cups = input.parse().expect("input should be cup labels");
      assert_eq!(3, cups.current_cup_label);
      assert_eq!(8, cups.decrement_label(9));
      assert_eq!(9, cups.decrement_label(1));
      assert_eq!(2, cups.choose_destination());
      assert_eq!(5, cups.clockwise_of(2));
      assert_eq!(7, cups.clockwise_of(6));
      assert_eq!(3, cups.clockwise_of(7));
      cups.pick_up_one();
      assert_eq!(1, cups.picked_up.len());
      assert_eq!(9, cups.clockwise_of(3));
      cups.pick_up_one();
      assert_eq!(1, cups.picked_up.len());
      assert_eq!(9, cups.clockwise_of(3));
    }
}
