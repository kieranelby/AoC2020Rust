use std::collections::HashMap;

const IMAGE_WIDTH: usize = 12;

fn main() {
  let content = std::fs::read_to_string("input.txt").expect("could not read file");
  let mut lines = content.lines();
  let mut all_tiles_by_key = HashMap::new();
  let mut variant_keys_by_base_key = HashMap::new();
  let mut side_code_counts = HashMap::new();
  loop {
    match read_tile(&mut lines) {
      None => { break; },
      Some(tile) => {
        let variants = make_variants(&tile);
        let mut variant_keys = Vec::new();
        for variant in variants {
          let variant_key = variant.key;
          variant_keys.push(variant_key);
          for &side in &SIDES {
            *side_code_counts.entry(variant.side_code(side)).or_insert(0) += 1;
          }
          all_tiles_by_key.insert(variant_key, variant);
          //println!("{:?}", variant_key);
        }
        variant_keys_by_base_key.insert(tile.key, variant_keys);
      }
    }
  }
  let base_keys : Vec<TileKey> = variant_keys_by_base_key.keys().map(|k| *k).collect();
  let context = Context {
    all_tiles_by_key : &all_tiles_by_key,
    variant_keys_by_base_key : &variant_keys_by_base_key,
    base_keys : &base_keys,
    side_code_counts: &side_code_counts
  };

  let num_base_tiles = variant_keys_by_base_key.len();
  println!("num tiles = {}", num_base_tiles);
  assert_eq!(num_base_tiles, IMAGE_WIDTH*IMAGE_WIDTH);
  println!("num variants = {}", all_tiles_by_key.len());
  //println!("sanity check: {:?}", all_tiles.get(&TileKey { id: 1171, flip: Flip::Vertical, rotate: Rotate::Ninety}));

  let mut candidates : Vec<Image> = Vec::new();
  candidates.push(Image::new());
  for pass in 0..IMAGE_WIDTH*IMAGE_WIDTH {
    println!("pass: {}, candidates: {}", pass, candidates.len());
    candidates = search(&context, pass, &candidates);
    if candidates.len() == 0 {
      break;
    }
    //let example_candidate = candidates.iter().next().unwrap();
    //println!("example candidate: ");
    //example_candidate.print(&context);
  }
  let pt1_solution =
    candidates
    .iter()
    .next()
    .expect("no solution found");
  println!("pt1 solution:");
  pt1_solution.print(&context);
  println!("pt1 solution corner product = {:?}", 
    pt1_solution
    .corners(&context)
    .iter()
    .map(|t| t.key.id as u64)
    .fold(1, |a,b| a*b));

  let mut actual_image_data = make_blank_tile_data((TILE_SIZE - 2) * IMAGE_WIDTH);
  for tile_row in 0..IMAGE_WIDTH {
    for tile_col in 0..IMAGE_WIDTH {
      let tile_key = pt1_solution.rows_of_tiles[tile_row][tile_col].unwrap();
      let tile = context.all_tiles_by_key.get(&tile_key).unwrap();
      for pixel_row in 0..TILE_SIZE-2 {
        for pixel_col in 0..TILE_SIZE-2 {
          let dst_row = tile_row * (TILE_SIZE-2) + pixel_row;
          let dst_col = tile_col * (TILE_SIZE-2) + pixel_col;
          actual_image_data[dst_row][dst_col] = tile.data[pixel_row+1][pixel_col+1];
        }
      }
    }
  }
  let mut orientations = Vec::new();
  orientations.push(actual_image_data.clone());
  orientations.push(rotate_data_right(&actual_image_data));
  orientations.push(rotate_data_right(&rotate_data_right(&actual_image_data)));
  orientations.push(rotate_data_right(&rotate_data_right(&rotate_data_right(&actual_image_data))));
  orientations.push(flip_data_horizontal(&actual_image_data));
  orientations.push(rotate_data_right(&flip_data_horizontal(&actual_image_data)));
  orientations.push(rotate_data_right(&rotate_data_right(&flip_data_horizontal(&actual_image_data))));
  orientations.push(rotate_data_right(&rotate_data_right(&rotate_data_right(&flip_data_horizontal(&actual_image_data)))));
  for orientation in &orientations {
    const SEA_MONSTER_LEN : usize = 20;
    let sea_monster_top    = "                  # ";
    let sea_monster_middle = "#    ##    ##    ###";
    let sea_monster_bottom = " #  #  #  #  #  #   ";
    let mut sea_monsters = Vec::new();
    for row in 1..orientation.len()-1 {
      for col in 1..orientation.len()-SEA_MONSTER_LEN {
        if check_set_in_data(orientation, sea_monster_middle, row, col) &&
           check_set_in_data(orientation, sea_monster_top, row-1, col) &&
           check_set_in_data(orientation, sea_monster_bottom, row+1, col) {
          sea_monsters.push((row, col));
          println!("woot woot");
        }
      }
    }
    if sea_monsters.len() > 0 {
      let mut monsterless = orientation.clone();
      for (row, col) in sea_monsters {
        clear_in_data(&mut monsterless, sea_monster_middle, row, col);
        clear_in_data(&mut monsterless, sea_monster_top, row-1, col);
        clear_in_data(&mut monsterless, sea_monster_bottom, row+1, col);
      }
      print_image_data(&monsterless);
      println!("pt2={}",
        monsterless
        .iter()
        .map(|r| r.iter().map(|c| if *c {1} else {0}).sum::<usize>())
        .sum::<usize>())
    }
  }
}

fn check_set_in_data(tile_data : &TileData, pattern : &str, row : usize, col : usize) -> bool {
  let mut col = col;
  let line = &tile_data[row];
  for c in pattern.chars() {
    if c == '#' && !line[col] { return false; }
    col += 1;
  }
  true
}

fn clear_in_data(tile_data : &mut TileData, pattern : &str, row : usize, col : usize) {
  let mut col = col;
  let line = &mut tile_data[row];
  for c in pattern.chars() {
    if c == '#' { line[col] = false; }
    col += 1;
  }
}

struct Context<'a> {
  all_tiles_by_key : &'a HashMap<TileKey,Tile>,
  variant_keys_by_base_key : &'a HashMap<TileKey,Vec<TileKey>>,
  base_keys : &'a Vec<TileKey>,
  side_code_counts : &'a HashMap<SideCode,usize>,
}

fn search(context : &Context, pass : usize, images : &Vec<Image>) -> Vec<Image> {
  let mut next_images : Vec<Image> = Vec::new();
  let col = pass % IMAGE_WIDTH;
  let row = pass / IMAGE_WIDTH;
  for image in images {

    for base_key in context.base_keys {
      // each tile can only be used once
      if image.got_base_key(base_key) {
        continue;
      }
      for variant_key in context.variant_keys_by_base_key.get(base_key).unwrap() {
        let tile = context.all_tiles_by_key.get(variant_key).unwrap();
        match image.with(context, tile, row, col) {
          None => continue,
          Some(new_image) => next_images.push(new_image)
        }
      }
    }
  }
  next_images
}

// oddly we don't need vertical because horizontal + rotate is enough
#[derive(Debug,Copy,Clone,PartialEq,PartialOrd,Eq,Ord,Hash)]
enum Flip {
  None, Horizontal
}
static FLIPS : [Flip; 2] = [Flip::None, Flip::Horizontal];

#[derive(Debug,Copy,Clone,PartialEq,PartialOrd,Eq,Ord,Hash)]
enum Rotate {
  None, Ninety, OneEighty, TwoSeventy
}
static ROTATES : [Rotate; 4] = [Rotate::None, Rotate::Ninety, Rotate::OneEighty, Rotate::TwoSeventy];

#[derive(Debug,Copy,Clone,PartialEq,PartialOrd,Eq,Ord,Hash)]
struct TileKey {
  pub id : u32,
  pub flip: Flip,
  pub rotate: Rotate
}

#[derive(Debug,Copy,Clone,PartialEq,PartialOrd,Eq,Ord,Hash)]
enum Side {
  Top,Right,Bottom,Left
}
static SIDES : [Side; 4] = [Side::Top, Side::Right, Side::Bottom, Side::Left];

struct Image {
  rows_of_tiles : Vec<Vec<Option<TileKey>>>
}

impl Image {
  fn new() -> Image {
    let mut rows_of_tiles = Vec::new();
    rows_of_tiles.reserve(IMAGE_WIDTH);
    for _row in 0..IMAGE_WIDTH {
      let mut tile_keys = Vec::new();
      tile_keys.reserve(IMAGE_WIDTH);
      for _col in 0..IMAGE_WIDTH {
        tile_keys.push(None);
      }
      rows_of_tiles.push(tile_keys);
    }
    Image { rows_of_tiles : rows_of_tiles }
  }

  fn got_base_key(self: &Self, base_key : &TileKey) -> bool {
    self.rows_of_tiles.iter().any(|r| r.iter().any(|mt| match mt { 
      Some(tk) => tk.id == base_key.id, None => false
    }))
  }

  fn with(self : &Self, context: &Context, tile : &Tile, row: usize, col: usize) -> Option<Image> {

    // if it's in an outer edge, check the outer border is not capable of matching anything ever
    // otherwise check the inner borders are capable of matching something eventually
    if row == 0 && !side_code_unique(context, tile, Side::Top) { 
      return None;
    }
    if row > 0 && side_code_unique(context, tile, Side::Top) {
      return None;
    }
    if row == IMAGE_WIDTH-1 && !side_code_unique(context, tile, Side::Bottom) {
      return None;
    }
    if row < IMAGE_WIDTH-1 && side_code_unique(context, tile, Side::Bottom) {
      return None;
    }
    if col == 0 && !side_code_unique(context, tile, Side::Left) {
      return None;
    }
    if col > 0 && side_code_unique(context, tile, Side::Left) {
      return None;
    }
    if col == IMAGE_WIDTH-1 && !side_code_unique(context, tile, Side::Right) {
      return None;
    }
    if col < IMAGE_WIDTH-1 && side_code_unique(context, tile, Side::Right) {
      return None;
    }

    // check inner borders
    if col > 0 {
      if let Some(tile_key_left) = self.rows_of_tiles[row][col - 1] {
        let tile_left = context.all_tiles_by_key.get(&tile_key_left).unwrap();
        if tile_left.side_code(Side::Right) != tile.side_code(Side::Left) {
          return None;
        }
      }
    }
    if col < IMAGE_WIDTH-1 {
      if let Some(tile_key_right) = self.rows_of_tiles[row][col + 1] {
        let tile_right = context.all_tiles_by_key.get(&tile_key_right).unwrap();
        if tile_right.side_code(Side::Left) != tile.side_code(Side::Right) {
          return None;
        }
      }
    }
    if row > 0 {
      if let Some(tile_key_above) = self.rows_of_tiles[row-1][col] {
        let tile_above = context.all_tiles_by_key.get(&tile_key_above).unwrap();
        if tile_above.side_code(Side::Bottom) != tile.side_code(Side::Top) {
          return None;
        }
      }
    }
    if row < IMAGE_WIDTH-1 {
      if let Some(tile_key_below) = self.rows_of_tiles[row+1][col] {
        let tile_below = context.all_tiles_by_key.get(&tile_key_below).unwrap();
        if tile_below.side_code(Side::Top) != tile.side_code(Side::Bottom) {
          return None;
        }
      }
    }

    // weirdly clone is not working as I expected
    // let mut new_rows_of_tiles = self.rows_of_tiles.clone();
    let mut new_image = self.clone();
    new_image.rows_of_tiles[row][col] = Some(tile.key);
    Some(new_image)
  }

  fn corners<'a,'b>(self: &'a Self, context : &'b Context) -> Vec<&'b Tile> {
    [(0, 0),(0, IMAGE_WIDTH-1),(IMAGE_WIDTH-1,0),(IMAGE_WIDTH-1, IMAGE_WIDTH-1)]
    .iter()
    .map(|(row,col)| self.rows_of_tiles[*row][*col].expect("corner not set yet"))
    .map(|tk| context.all_tiles_by_key.get(&tk).unwrap())
    .collect()
  }

  fn print(self: &Self, context : &Context) {
    for row in 0..IMAGE_WIDTH {
      for col in 0..IMAGE_WIDTH {
        match self.rows_of_tiles[row][col] {
          None => {
            print!("Tile ????: ");
          }
          Some(tile_key) => {
            print!("Tile {:04}: ", tile_key.id);
          }
        }
      }
      println!();
      for pixel_row in 0..TILE_SIZE {
        for col in 0..IMAGE_WIDTH {
          match self.rows_of_tiles[row][col] {
            None => {
              print!("?????????? ");
            }
            Some(tile_key) => {
              let tile = context.all_tiles_by_key.get(&tile_key).unwrap();
              print!("{} ", tile.data[pixel_row].iter().map(|b| if *b {'#'} else {'.'}).collect::<String>());
            }
          }
        }
        println!();
      }
    }
  }
}

impl Clone for Image {
  fn clone (&self) -> Self {
    let mut new_image = Image::new();
    for row in 0..IMAGE_WIDTH {
      for col in 0..IMAGE_WIDTH {
        new_image.rows_of_tiles[row][col] = self.rows_of_tiles[row][col];
      }
    }
    new_image
  }
}

// is this too unique?
fn side_code_unique(context: &Context, tile : &Tile, side : Side) -> bool {
  let side_code = tile.side_code(side);
  let count = *context.side_code_counts.get(&side_code).unwrap();
  // TODO - what if symmetric side? should this be 8 or something?
  // TODO - why is it 4 anyway?
  count == 4
}

const TILE_SIZE: usize = 10;
type TileData = Vec<Vec<bool>>;

type SideCode = u32;

#[derive(Debug,Clone)]
struct Tile {
  pub key: TileKey,
  pub data: TileData,
  pub side_codes: [SideCode;4] // T,R,B,L
}

impl Tile {
  fn side_code(self : &Self, side : Side) -> SideCode {
    match side {
      Side::Top => self.side_codes[0],
      Side::Right => self.side_codes[1],
      Side::Bottom => self.side_codes[2],
      Side::Left => self.side_codes[3],
    }
  }
}

fn read_tile<'a, I>(lines : &mut I) -> Option<Tile> where I : Iterator<Item=&'a str> {
  let tile_id : u32;
  loop {
  let maybe_header = lines.next();
    match maybe_header {
      None => return None,
      Some(header_text) => {
        if header_text.len() == 0 {
          continue; // skip blank lines
        }
        let header_digits = header_text.chars().filter(|c| c.is_digit(10)).collect::<String>();
        tile_id = header_digits.parse().expect("could not find tile id digits");
        break;
      }
    }
  }
  let mut tile_data : TileData = make_blank_tile_data(TILE_SIZE);
  for row in 0..TILE_SIZE {
    let data_line = lines.next().expect("insuffucient data lines");
    assert_eq!(data_line.len(), TILE_SIZE, "unexpected data line length");
    let mut col = 0;
    for c in data_line.chars() {
      match c {
        '#' => { tile_data[row][col] = true; },
        '.' => {},
        _ => { panic!("unexpected tile content {}", c); }
      }
      col = col + 1;
    }
  }
  let side_codes = make_side_codes(&tile_data);
  Some(Tile {
    key: TileKey {
      id: tile_id,
      flip: Flip::None,
      rotate: Rotate::None
    },
    data: tile_data,
    side_codes: side_codes
  })
}

fn make_blank_tile_data(size : usize) -> TileData {
  let mut rows = Vec::new();
  rows.reserve(size);
  for _row in 0..size {
    let mut cols = Vec::new();
    cols.reserve(size);
    for _col in 0..size {
      cols.push(false);
    }
    rows.push(cols);
  }
  rows
}

fn make_variants(base : &Tile) -> Vec<Tile> {
  let mut variants = Vec::new();
  for &flip in &FLIPS {
    for &rotate in &ROTATES {
      // NB: we flip then rotate, i guess that still covers 'em all?
      variants.push(rotate_tile(&flip_tile(base,flip),rotate));
    }
  }
  return variants;
}

fn flip_tile(base : &Tile, flip : Flip) -> Tile {
  assert_eq!(base.key.flip, Flip::None);
  let data = {
    match flip {
      Flip::None => base.data.clone(),
      Flip::Horizontal => flip_data_horizontal(&base.data),
    }
  };
  let side_codes = make_side_codes(&data);
  Tile {
    key: TileKey { id: base.key.id, rotate: base.key.rotate, flip: flip },
    data: data,
    side_codes: side_codes
  }
}

fn rotate_tile(base : &Tile, rotate : Rotate) -> Tile {
  assert_eq!(base.key.rotate, Rotate::None);
  let data = match rotate {
    Rotate::None => base.data.clone(),
    Rotate::Ninety => rotate_data_right(&base.data),
    Rotate::OneEighty => rotate_data_right(&rotate_data_right(&base.data)),
    Rotate::TwoSeventy => rotate_data_right(&rotate_data_right(&rotate_data_right(&base.data))),
  };
  let side_codes = make_side_codes(&data);
  Tile {
    key: TileKey { id: base.key.id, rotate: rotate, flip: base.key.flip },
    data: data,
    side_codes: side_codes
  }
}

fn rotate_data_right(data : &TileData) -> TileData {
  let mut new_data : TileData = make_blank_tile_data(data.len());
  for row in 0..data.len() {
    for col in 0..data.len() {
      new_data[row][col] = data[data.len() - 1 - col][row];
    }
  }
  new_data
}

fn flip_data_horizontal(data : &TileData) -> TileData {
  let mut new_data : TileData = make_blank_tile_data(data.len());
  for row in 0..data.len() {
    for col in 0..data.len() {
      new_data[row][col] = data[row][data.len() - 1 - col];
    }
  }
  new_data
}

fn print_image_data(data : &TileData) {
  for line in data {
    println!("{}", line.iter().map(|b| if *b {'#'} else {'.'}).collect::<String>());
  }
}

fn make_side_codes(data : &TileData) -> [SideCode;4] {
  let mut side_codes = [0;4];
  let mut exponent : SideCode = 1;
  for idx in 0..TILE_SIZE {
    if data[0][idx] {
      side_codes[0] += exponent; // top
    }
    if data[idx][TILE_SIZE - 1] {
      side_codes[1] += exponent; // right
    }
    if data[TILE_SIZE - 1][idx] {
      side_codes[2] += exponent; // bottom
    }
    if data[idx][0] {
      side_codes[3] += exponent; // left
    }
    exponent = exponent * 2;
  }
  return side_codes;
}

fn reverse_side_code(side_code : SideCode) -> SideCode {
  let mut rev_side_code : SideCode = 0;
  for src_pos in 0..TILE_SIZE {
    let dst_pos = TILE_SIZE - 1 - src_pos;
    let src_exp = 2u32.pow(src_pos as u32);
    if side_code & src_exp != 0 {
      let dst_exp = 2u32.pow(dst_pos as u32);
      rev_side_code = rev_side_code + dst_exp;
    }
  }
  return rev_side_code;
}
