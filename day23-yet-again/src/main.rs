use std::fmt;
use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

const NUM_CUPS : CupLabel = 1_000_000;
//const NUM_CUPS : CupLabel = 9;

fn main() {
  let input = "476138259";
  //let input = "389125467";
  //let num_moves = 10;
  //let num_moves = 100;
  let num_moves = 10_000_000;
  let mut cups = Cups::from_str(input);
  for move_num in 1..=num_moves {
    if move_num < 100 || move_num % 1000 == 0 {
      println!("-- move {} --", move_num);
    }
    cups.choose_three();
    let destination = cups.choose_destination();
    if cups.len() < 50 && move_num <= 100 {
      println!("{}", cups);
      println!("destination: {}", Cups::label_of(&destination));
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

type CupLabel = u32;

type WeakCupRef = Weak<RefCell<Cup>>;

#[derive(Debug)]
struct Cup {
  label: CupLabel,
  // doubly-linked circular structure
  prev: WeakCupRef,
  next: WeakCupRef,
  // will be None for the very lowest cup
  // a cup labelled N is owned by the cup labelled N+1
  lower: Option<Rc<RefCell<Cup>>>
}

#[derive(Debug)]
struct Cups {
  current_cup: WeakCupRef,
  highest_cup: Rc<RefCell<Cup>>,
  picked_up: Vec<WeakCupRef>
}

impl Cups {

  fn len(self : &Self) -> CupLabel {
    NUM_CUPS
  }

  fn new() -> Cups {
    let mut cup = Rc::new(RefCell::new(Cup {
      label: 1,
      prev: Weak::new(),
      next: Weak::new(),
      lower: None,
    }));
    let first_cup = Rc::clone(&cup);
    for label in 2..=NUM_CUPS {
      let next_cup = Rc::new(RefCell::new(Cup {
        label: label,
        prev: Rc::downgrade(&cup),
        next: Weak::new(),
        lower: Some(Rc::clone(&cup)),
      }));
      {
        let mut inner_cup = cup.borrow_mut();
        inner_cup.next = Rc::downgrade(&next_cup);
      }
      cup = next_cup;
    }
    let last_cup = cup;
    {
      let mut inner_first_cup = first_cup.borrow_mut();
      inner_first_cup.prev = Rc::downgrade(&last_cup);
    }
    {
      let mut inner_last_cup = last_cup.borrow_mut();
      inner_last_cup.next = Rc::downgrade(&first_cup);
    }
    Cups {
      current_cup: Rc::downgrade(&first_cup),
      highest_cup: last_cup,
      picked_up: Vec::new(),
    }
  }

  fn from_str(s : &str) -> Cups {
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

  fn find_and_move_to_current(self : &mut Self, cup_label: CupLabel) {
    if self.current_cup.upgrade().unwrap().borrow().label == cup_label {
      return;
    }
    let cup = self.position_of(cup_label);
    Cups::temporarily_remove(Weak::clone(&cup));
    Cups::reinsert_before(Weak::clone(&cup), Weak::clone(&self.current_cup));
    self.current_cup = cup;
  }


  fn position_of(self : &Self, cup_label: CupLabel) -> WeakCupRef {
    let mut position = Weak::clone(&self.current_cup);
    let original_position = Weak::clone(&position);
    loop {
      if Cups::label_of(&position) == cup_label {
        return Weak::clone(&position);
      }
      position = Weak::clone(&position.upgrade().unwrap().borrow().next);
      if Cups::label_of(&position) == Cups::label_of(&original_position) {
        panic!("not found: {}", cup_label);
      }
    }
  }

  fn temporarily_remove(cup : WeakCupRef) {
    let prev = Weak::clone(&cup.upgrade().unwrap().borrow().prev);
    let next = Weak::clone(&cup.upgrade().unwrap().borrow().next);
    {
      prev.upgrade().unwrap().borrow_mut().next = Weak::clone(&next);
      next.upgrade().unwrap().borrow_mut().prev = Weak::clone(&prev);
    }
    {
      cup.upgrade().unwrap().borrow_mut().next = Weak::new();
      cup.upgrade().unwrap().borrow_mut().prev = Weak::new();
    }
  }

  fn reinsert_before(unlinked_cup : WeakCupRef, destination : WeakCupRef) {
    let prev = Weak::clone(&destination.upgrade().unwrap().borrow().prev);
    unlinked_cup.upgrade().unwrap().borrow_mut().prev = Weak::clone(&prev);
    unlinked_cup.upgrade().unwrap().borrow_mut().next = Weak::clone(&destination);
    prev.upgrade().unwrap().borrow_mut().next = Weak::clone(&unlinked_cup);
    destination.upgrade().unwrap().borrow_mut().prev = Weak::clone(&unlinked_cup);
  }

  fn label_of(cup : &WeakCupRef) -> CupLabel {
    cup.upgrade().unwrap().borrow().label
  }

  fn label_clockwise_of(self : &Self, cup_label: CupLabel) -> CupLabel {
    Cups::label_of(&Cups::clockwise_of(&self.position_of(cup_label)))
  }

  fn clockwise_of(cup : &WeakCupRef) -> WeakCupRef {
    Weak::clone(&cup.upgrade().unwrap().borrow_mut().next)
  }

  fn choose_three(self : &mut Self) {
    for _ in 0..3 {
      self.choose_one();
    }
  }

  fn choose_one(self : &mut Self) {
    assert!(self.len() as usize - self.picked_up.len() > 1);
    let removed = Cups::clockwise_of(&self.current_cup);
    Cups::temporarily_remove(Weak::clone(&removed));
    self.picked_up.push(removed);
  }

  fn choose_destination(self : &Self) -> WeakCupRef {
    let mut destination = Weak::clone(&self.current_cup);
    loop {
      destination = Rc::downgrade(match &destination.upgrade().unwrap().borrow().lower {
        None => &self.highest_cup,
        Some(lower) => &lower,
      });
      if !self.picked_up.iter().any(|cr| Cups::label_of(cr) == Cups::label_of(&destination)) {
        return destination;
      }
    }
  }

  fn move_chosen(self : &mut Self, destination : WeakCupRef) {
    let mut target = Cups::clockwise_of(&destination);
    for cup in self.picked_up.iter().rev() {
      Cups::reinsert_before(Weak::clone(&cup), target);
      target = Weak::clone(&cup);
    }
    self.picked_up.clear();
  }

  fn select_new_current_cup(self : &mut Self) {
    self.current_cup = Cups::clockwise_of(&self.current_cup);
  }

  fn labels_after(self : &Self, cup_label : CupLabel) -> String {
    let mut s = String::new();
    let mut position = self.position_of(cup_label);
    let original_position = Weak::clone(&position);
    loop {
      position = Weak::clone(&position.upgrade().unwrap().borrow().next);
      if Cups::label_of(&position) == Cups::label_of(&original_position) {
        break;
      }
      s.push_str(&Cups::label_of(&position).to_string());
    }
    s
  }
}

// we have to do this to avoid overflowing the stack for large sizes with the default recursive impl
impl Drop for Cups {
  fn drop(&mut self) {
    if !self.picked_up.is_empty() {
      self.move_chosen(Weak::clone(&self.current_cup));
    }
    let mut maybe_cup = Some(Rc::clone(&self.highest_cup));
    while let Some(cup) = maybe_cup {
      let next_cup = match &cup.borrow().lower {
        None => None,
        Some(lower) => Some(Rc::clone(&lower))
      };
      cup.borrow_mut().lower = None;
      maybe_cup = next_cup;
    }
  }
}

impl fmt::Display for Cups {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "cups:")?;
    let mut position = Weak::clone(&self.current_cup);
    let original_position = Weak::clone(&position);
    loop {
      write!(f, " {}", Cups::label_of(&position))?;
      position = Weak::clone(&position.upgrade().unwrap().borrow().next);
      if Cups::label_of(&position) == Cups::label_of(&original_position) {
        break;
      }
    }
    if !self.picked_up.is_empty() {
      write!(f, "; pick up:")?;
      for c in &self.picked_up {
        write!(f, " {}", c.upgrade().unwrap().borrow().label)?;
      }
    }
    Ok(())
  }
}
