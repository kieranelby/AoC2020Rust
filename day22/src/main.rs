use std::collections::VecDeque;

fn main() {
  let content = std::fs::read_to_string("input.txt").expect("could not read file");
  play(false, &content);
  play(true, &content);
}

fn play(allow_recurse : bool, content : &str) {
  let mut decks = read_starting_decks(content.lines());
  println!("start: {:?}", decks);
  let (score, _winning_player_index) = play_combat(allow_recurse, &mut decks);
  println!("end: {:?}", decks);
  println!("score: {:?}", score);
}

type Card = u32;

fn play_combat(allow_recurse : bool, decks : &mut Vec<VecDeque<Card>>) -> (u64,usize) {
  let total_num_cards : usize = decks.iter().map(|d| d.len()).sum();
  let winning_player_index : usize;
  let mut history : Vec<String> = Vec::new();
  loop {
    // have we seen these decks before?
    let fingerprint = format!("{:?}", decks);
    if history.contains(&fingerprint) {
      winning_player_index = 0;
      break;
    }
    history.push(fingerprint);
    // peek at the top cards
    let player_index_and_top_card : Vec<(usize,Card)> =
      decks.iter().map(|d| *d.get(0).expect("someone out of cards")).enumerate().collect();
    let mut player_index_and_top_card_winner_first = player_index_and_top_card.clone();
    // has every player got enough cards to recurse?
    let round_winning_player_index = if allow_recurse && player_index_and_top_card.iter().all(|(player_index, top_card)| {
      const DRAWN_BUT_NOT_TAKEN : usize = 1;
      let remaining = decks[*player_index].len() - DRAWN_BUT_NOT_TAKEN;
      remaining >= *top_card as usize
    }) {
      // recurse
      let mut sub_decks : Vec<VecDeque<Card>> = player_index_and_top_card.iter().map(|(player_index,top_card)| {
        let deck = &decks[*player_index];
        let mut sub_deck = VecDeque::new();
        // can't slice deques ...
        for index in 1..=*top_card as usize {
          sub_deck.push_back(deck[index]);
        }
        sub_deck
      }).collect();
      //println!("start recursing >>>");
      let (_score, sub_winner) = play_combat(allow_recurse, &mut sub_decks);
      //println!("end recursing <<<<");
      player_index_and_top_card_winner_first.sort_by(|(pa,_ca),(pb,_cb)| {
        let wa = if *pa == sub_winner { true } else { false };
        let wb = if *pb == sub_winner { true } else { false };
        wb.cmp(&wa)
      });
      sub_winner
    } else {
      // player with highest card wins
      player_index_and_top_card_winner_first.sort_by(|(_pa,ca),(_pb,cb)| cb.cmp(ca));
      player_index_and_top_card_winner_first.iter().map(|(pi,_tc)| *pi).next().expect("round should have a winner")
    };
    //println!("player {} wins", round_winning_player_index + 1);
    // remove the top cards and award them to the winner (winning first)
    let mut player_index = 0;
    for deck in decks.iter_mut() {
      deck.pop_front();
      if player_index == round_winning_player_index {
        deck.extend(player_index_and_top_card_winner_first.iter().map(|(_pi,tc)| tc));
      }
      player_index += 1;
    }
    //println!("decks: {:?}", decks);
    // has someone got all the cards?
    match decks.iter().enumerate().filter(|(_pi,d)| d.len() == total_num_cards).next() {
      Some((player_index, _deck)) => { 
        winning_player_index = player_index;
        break;
      },
      _ => {}
    }
  }
  let winning_deck = &decks[winning_player_index];
  (compute_score(winning_deck.iter().map(|c| *c)), winning_player_index)
}

fn compute_score(deck : impl DoubleEndedIterator<Item = Card>) -> u64 {
  deck.rev().fold((0u64, 1u64), |(score,mul), card| {
    (score + card as u64 * mul, mul + 1)
  }).0
}

fn read_starting_decks<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<VecDeque<u32>> {
  let mut decks = Vec::new();
  let mut player_index = None;
  for line in lines {
    if line.len() == 0 {
      // no-op
    } else if line.starts_with("Player") {
      decks.push(VecDeque::new());
      player_index = Some(decks.len() - 1);
    } else {
      let card = line.parse::<u32>().unwrap();
      decks[player_index.expect("don't know which player this card is for")].push_back(card);
    }
  }
  return decks;
}