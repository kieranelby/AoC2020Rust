use std::fmt;

const NUM_CUPS : usize = 1_000_000;
//const NUM_CUPS : usize = 9;

static mut CUPS_RING : [CupLabel;NUM_CUPS] = [0;NUM_CUPS];

fn main() {
  //let input = "389125467";
  let input = "476138259";
  //let num_moves = 10;
  //let num_moves = 100;
  let num_moves = 10_000_000;
  let mut cups = Cups::new(input);
  for move_num in 1..=num_moves {
    if move_num < 100 || move_num % 100 == 0 {
      println!("-- move {} --", move_num);
    }
    cups.choose_three();
    let destination = cups.choose_destination();
    if cups.len() < 50 && move_num <= 100 {
      println!("{}", cups);
      println!("destination: {}", destination);
  }
    cups.move_chosen(destination);
    cups.select_new_current_cup();
  }
  println!("-- final --");
  if cups.len() <= 10 {
    println!("{}", cups);
    println!("labels after: {}", cups.labels_after(1));
  }
  let c1 = cups.clockwise_of(1);
  let c2 = cups.clockwise_of(c1);
  let cp = c1  as u64 * c2  as u64;
  println!("{} * {} = {}", c1, c2, cp);
}

type CupLabel = u32;

#[derive(Debug)]
struct Cups<'a> {
  ring: &'a mut [CupLabel;NUM_CUPS],
  current_index: usize,
  picked_up: Vec<CupLabel>,
  lowest_label: CupLabel,
  highest_label: CupLabel
}

impl Cups<'_> {

  fn new(s : &str) -> Cups {
    let clockwise_cup_labels =
      s
      .chars()
      .map(|c| c.to_string().parse::<CupLabel>().expect("cup labels should be small non-negative numbers"))
      .collect::<Vec<CupLabel>>();
    unsafe {
      for i in 1..=NUM_CUPS {
        CUPS_RING[i-1] = i as CupLabel;
      }
      let mut i = 0;
      for &cup in &clockwise_cup_labels {
        CUPS_RING[i] = cup;
        i += 1;
      }
      Cups {
        ring: &mut CUPS_RING,
        current_index: 0,
        picked_up: Vec::new(),
        lowest_label: 1,
        highest_label: NUM_CUPS as CupLabel,
      }
    }
  }

  fn position_of(self : &Self, cup_label: CupLabel) -> usize {
    self.ring.iter().position(|c| *c == cup_label).unwrap()
  }
  
  #[inline(always)]
  fn len(self : &Self) -> usize {
    NUM_CUPS
  }

  #[inline(always)]
  fn increment_position(self : &Self, position: usize) -> usize {
    if position == self.len() - 1 {
      0
    } else {
      position + 1
    }
  }

  fn clockwise_of(self : &Self, cup_label: CupLabel) -> CupLabel {
    self.ring[self.increment_position(self.position_of(cup_label))]
  }

  fn choose_three(self : &mut Self) {
    let mut src = self.current_index;
    for _ in 0..3 {
      src = self.increment_position(src);
      self.picked_up.push(self.ring[src]);
    }
  }

  fn decrement_label(self : &Self, cup_label: CupLabel) -> CupLabel {
    if cup_label == self.lowest_label {
      self.highest_label
    } else {
      cup_label - 1
    }
  }

  fn choose_destination(self : &Self) -> CupLabel {
    let mut destination = self.ring[self.current_index];
    loop {
      destination = self.decrement_label(destination);
      if !self.picked_up.contains(&destination) {
        return destination;
      }
    }
  }

  fn move_chosen(self : &mut Self, destination : CupLabel) {

    let ring_len = self.len();
    let pick_len = self.picked_up.len();
    let original_destination_position = self.position_of(destination);
    let mut dst = self.current_index;
    let mut src = dst;
    for _ in 0..pick_len {
      src = self.increment_position(src);
    }
    let mut size_left = if original_destination_position >= src {
      original_destination_position - src
    } else {
      ring_len - (src - original_destination_position)
    };

    while size_left > 0 {
      const STRIDE : usize = 1024;
      if STRIDE < size_left && dst + STRIDE < ring_len && src + STRIDE < ring_len {
        self.ring.copy_within(src+1..src+STRIDE+1,dst+1);
        dst += STRIDE;
        src += STRIDE;
        size_left -= STRIDE;
      } else {
        // yeah i don't know why i did the increment first
        dst = self.increment_position(dst);
        src = self.increment_position(src);
        self.ring[dst] = self.ring[src];
        size_left -= 1;
      }
    }

    for &cup in &self.picked_up {
      dst = self.increment_position(dst);
      self.ring[dst] = cup;
    }

    self.picked_up.clear();
  }

  fn is_current_at_end(self : &Self) -> bool {
    self.current_index == self.len() - 1
  } 

  fn select_new_current_cup(self : &mut Self) {
    self.current_index = if self.is_current_at_end() {
      0
    } else {
      self.current_index + 1
    };
  }

  fn labels_after(self : &Self, cup_label : CupLabel) -> String {
    let mut s = String::new();
    let mut position = self.position_of(cup_label);
    let original_position = position;
    loop {
      position = self.increment_position(position);
      if position == original_position {
        break;
      }
      s.push_str(&self.ring[position].to_string());
    }
    s
  }

}

impl fmt::Display for Cups<'_> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "cups:")?;
    let mut position = self.current_index;
    write!(f, " ({})", self.ring[position])?;
    loop {
      position = self.increment_position(position);
      if position == self.current_index {
        break;
      }
      write!(f, " {}", self.ring[position])?;
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
