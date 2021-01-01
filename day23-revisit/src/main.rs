use std::fmt;

const NUM_CUPS : CupLabel = 1_000_000;
//const NUM_CUPS : CupLabel = 9;

fn main() {
  let input = "476138259";
  //let input = "389125467";
  //let num_moves = 10;
  //let num_moves = 100;
  let num_moves = 10_000_000;
  unsafe {
    let mut cups = Cups::from_str(input);
    for move_num in 1..=num_moves {
      if move_num < 100 || move_num % 1000 == 0 {
        println!("-- move {} --", move_num);
      }
      cups.choose_three();
      let destination = cups.choose_destination();
      if cups.len() < 50 && move_num <= 100 {
        println!("{}", cups);
        println!("destination: {}", (*destination).label);
      }
      cups.move_chosen(destination);
      cups.select_new_current_cup();
    }
    println!("-- final --");
    if cups.len() <= 10 {
      println!("{}", cups);
      println!("labels after: {}", cups.labels_after(1));
    }
    let c1 = cups.label_clockwise_of(1);
    let c2 = cups.label_clockwise_of(c1);
    let cp = c1  as u64 * c2  as u64;
    println!("{} * {} = {}", c1, c2, cp);
  }
}

type CupLabel = u32;

type CupRef = *mut Cup;

#[derive(Debug)]
struct Cup {
  label: CupLabel,
  prev: CupRef,
  next: CupRef,
  lower: CupRef
}

#[derive(Debug)]
struct Cups {
  current_cup: CupRef,
  highest_cup: CupRef,
  picked_up: Vec<CupRef>
}

impl Drop for Cups {
  fn drop(&mut self) {
    unsafe {
      if !self.picked_up.is_empty() {
        self.move_chosen(self.current_cup);
      }
      let mut cup = self.highest_cup;
      while !cup.is_null() {
        let next_cup = (*cup).lower;
        let _junk = Box::from_raw(cup);
        cup = next_cup;
      }
      self.current_cup = std::ptr::null_mut();
      self.highest_cup = std::ptr::null_mut();
    }
  }
}

impl Cups {

  unsafe fn new() -> Cups {
    let mut cup = Box::into_raw(Box::new(Cup {
      label: 1,
      prev: std::ptr::null_mut(),
      next: std::ptr::null_mut(),
      lower: std::ptr::null_mut(),
    }));
    let first_cup = cup;
    for label in 2..=NUM_CUPS {
      let next_cup = Box::into_raw(Box::new(Cup {
        label: label,
        prev: cup,
        next: std::ptr::null_mut(),
        lower: cup,
      }));
      (*cup).next = next_cup;
      cup = next_cup;
    }
    let last_cup = cup;
    (*first_cup).prev = last_cup;
    (*last_cup).next = first_cup;
    Cups {
      current_cup: first_cup,
      highest_cup: last_cup,
      picked_up: Vec::new(),
    }
  }

  unsafe fn from_str(s : &str) -> Cups {
    let clockwise_cup_labels =
      s
      .chars()
      .map(|c| c.to_string().parse::<CupLabel>().expect("cup labels should be small non-negative numbers"))
      .collect::<Vec<CupLabel>>();
    let mut cups = Cups::new();
    for &label in clockwise_cup_labels.iter().rev() {
      cups.find_and_move_to_current(label);
    }
    cups
  }

  unsafe fn position_of(self : &Self, cup_label: CupLabel) -> CupRef {
    let mut position = self.current_cup;
    let original_position = position;
    loop {
      if (*position).label == cup_label {
        return position;
      }
      position = (*position).next;
      if (*position).label == (*original_position).label {
        panic!("not found: {}", cup_label);
      }
    }
  }
  
  #[inline(always)]
  fn len(self : &Self) -> CupLabel {
    NUM_CUPS
  }

  unsafe fn label_clockwise_of(self : &Self, cup_label: CupLabel) -> CupLabel {
    (*Cups::clockwise_of(self.position_of(cup_label))).label
  }

  unsafe fn clockwise_of(cup : CupRef) -> CupRef {
    (*cup).next
  }

  unsafe fn choose_three(self : &mut Self) {
    for _ in 0..3 {
      self.choose_one();
    }
  }

  unsafe fn choose_one(self : &mut Self) {
    let removed = Cups::clockwise_of(self.current_cup);
    Cups::temporarily_remove(removed);
    self.picked_up.push(removed);
  }

  unsafe fn find_and_move_to_current(self : &mut Self, cup_label: CupLabel) {
    if (*self.current_cup).label == cup_label {
      return;
    }
    let cup = self.position_of(cup_label);
    Cups::temporarily_remove(cup);
    Cups::reinsert_before(cup, self.current_cup);
    self.current_cup = cup;
  }

  unsafe fn temporarily_remove(cup : CupRef) {
    let prev = (*cup).prev;
    let next = (*cup).next;
    (*prev).next = next;
    (*next).prev = prev;
    (*cup).next = std::ptr::null_mut();
    (*cup).prev = std::ptr::null_mut();
  }

  unsafe fn reinsert_before(unlinked_cup : CupRef, destination : CupRef) {
    let prev = (*destination).prev;
    (*unlinked_cup).prev = prev;
    (*unlinked_cup).next = destination;
    (*prev).next = unlinked_cup;
    (*destination).prev = unlinked_cup;
  }

  unsafe fn choose_destination(self : &Self) -> CupRef {
    let mut destination = self.current_cup;
    loop {
      destination = (*destination).lower;
      if destination.is_null() {
        destination = self.highest_cup;
      }
      if !self.picked_up.iter().any(|cr| (**cr).label == (*destination).label) {
        return destination;
      }
    }
  }

  unsafe fn move_chosen(self : &mut Self, destination : CupRef) {
    let mut target = Cups::clockwise_of(destination);
    for &cup in self.picked_up.iter().rev() {
      Cups::reinsert_before(cup, target);
      target = cup;
    }
    self.picked_up.clear();
  }

  unsafe fn select_new_current_cup(self : &mut Self) {
    self.current_cup = Cups::clockwise_of(self.current_cup);
  }

  unsafe fn labels_after(self : &Self, cup_label : CupLabel) -> String {
    let mut s = String::new();
    let mut position = self.position_of(cup_label);
    let original_position = position;
    loop {
      position = (*position).next;
      if (*position).label == (*original_position).label {
        break;
      }
      s.push_str(&(*position).label.to_string());
    }
    s
  }
}

impl fmt::Display for Cups {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "cups:")?;
    let mut position = self.current_cup;
    let original_position = position;
    unsafe {
      loop {
        write!(f, " {}", (*position).label)?;
        position = (*position).next;
        if (*position).label == (*original_position).label {
          break;
        }
      }
      if !self.picked_up.is_empty() {
        write!(f, "; pick up:")?;
        for &c in &self.picked_up {
          write!(f, " {}", (*c).label)?;
        }
      }
    }
    Ok(())
  }
}
