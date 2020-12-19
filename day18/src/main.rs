fn main() {
  let content = std::fs::read_to_string("input.txt").expect("could not read file");
  let sum1 = content.lines().fold(0, |total,line| {
    total + eval_str1(line)
  });
  println!("pt 1={}", sum1);
  let sum2 = content.lines().fold(0, |total,line| {
    total + eval_str2(line)
  });
  println!("pt 2={}", sum2);
}

fn lex(s : &str) -> Vec<Token> {
  let mut tokens : Vec<Token> = Vec::new();
  let mut slice = s;
  loop {
    let (maybe_token, next_slice) = next_token(slice);
    match maybe_token {
      None => { break; },
      Some(token) => { tokens.push(token); }
    }
    slice = next_slice;
  }
  return tokens;
}

fn next_token(s : &str) -> (Option<Token>, &str) {
  if s.len() == 0 {
    (None, s)
  } else {
    match s.chars().next().unwrap() {
      ' ' => next_token(&s[1..]),
      '+' => (Some(Token::Add), &s[1..]),
      '*' => (Some(Token::Mul), &s[1..]),
      '(' => (Some(Token::OpenParen), &s[1..]),
      ')' => (Some(Token::CloseParen), &s[1..]),
      c if c.is_digit(10) => {
        let non_digit_index = s.find(|c2 : char| !c2.is_digit(10));
        let (value_str, next_idx) = match non_digit_index {
          None => (s, s.len()),
          Some(idx) => (&s[..idx], idx)
        };
        (Some(Token::Num(value_str.parse::<i64>().unwrap())), &s[next_idx..])
      },
      other => { panic!("unknown char {}", other) }
    }
  }
}

fn parse1(ts : &Vec<Token>) -> Expr {
  let backwards_tokens : Vec<Token> = ts.iter().map(|t| {
    match t {
      Token::OpenParen => Token::CloseParen,
      Token::CloseParen => Token::OpenParen,
      same => *same
    }
  }).rev().collect();
  parse1_r(&backwards_tokens[..]).0
}

fn parse1_r(ts : &[Token]) -> (Expr, &[Token]) {
  let mut remaining = ts;
  let lhs = match remaining[0] {
    Token::Num(value) => {
      remaining = &remaining[1..];
      Expr::Num(value)
    },
    Token::OpenParen => {
      remaining = &remaining[1..];
      let (inner_lhs, new_remaining) = parse1_r(remaining);
      remaining = new_remaining;
      if remaining[0] != Token::CloseParen {
        panic!("expected close paren");
      }
      remaining = &remaining[1..];
      inner_lhs
    },
    other => panic!("unexpected token {:?}", other)
  };
  if remaining.len() == 0 || remaining[0] == Token::CloseParen {
    return (lhs, remaining);
  }
  let expr_ctor = match remaining[0] {
    Token::Add => Expr::Add,
    Token::Mul => Expr::Mul,
    other => panic!("unexpected token {:?}", other)
  };
  remaining = &remaining[1..];
  let (rhs, new_remaining) = parse1_r(remaining);
  remaining = new_remaining;
  (expr_ctor(Box::new(lhs), Box::new(rhs)), remaining)
}

fn parse2(ts : &Vec<Token>) -> Expr {
  // e.g.
  //     2 * 3 + ( 4 * 5 )
  // =>  2 3 4 5 * + *
  //     1 + 2 * 3 + 4 * 5 + 6
  // =>  1 2 + 3 4 + * 5 6 + *
  let postfix = shunting_yard(ts);
  let mut stack : Vec<Expr> = Vec::new();
  for token in postfix {
    match token {
      Token::Num(value) => {
        stack.push(Expr::Num(value))
      },
      Token::Add => {
        let lhs = stack.pop().unwrap();
        let rhs = stack.pop().unwrap();
        stack.push(Expr::Add(Box::new(lhs), Box::new(rhs)))
      },
      Token::Mul => {
        let lhs = stack.pop().unwrap();
        let rhs = stack.pop().unwrap();
        stack.push(Expr::Mul(Box::new(lhs), Box::new(rhs)))
      },
      _ => panic!()
    }
  }
  return stack.pop().unwrap();
}

fn shunting_yard(ts : &Vec<Token>) -> Vec<Token> {
  let mut output : Vec<Token> = Vec::new();
  let mut stack : Vec<Token> = Vec::new();
  for token in ts {
    match token {
      Token::Num(_) => { output.push(*token); },
      Token::Add | Token::Mul => {
        loop {
          match stack.pop() {
            None => { break ; }
            Some(stack_token) => {
              if stack_token == Token::OpenParen || stack_token.precedence() < token.precedence() {
                stack.push(stack_token);
                break;
              }
              output.push(stack_token);
            }
          }
        }
        stack.push(*token);
      },
      Token::OpenParen => {
        stack.push(*token)
      },
      Token::CloseParen => {
        loop {
          match stack.pop() {
            None => { break ; }
            Some(token) => {
              if token == Token::OpenParen {
                break;
              }
              output.push(token);
            }
          }
        }
      }
    }
  }
  loop {
    match stack.pop() {
      None => { break ; }
      Some(token) => {
        if token == Token::OpenParen {
          panic!("mismatched parens");
        }
        output.push(token);
      }
    }
  }
  return output;
}

fn eval(ex : &Expr) -> i64 {
  match ex {
    Expr::Num(value) => { *value },
    Expr::Add(be1,be2) => {
      eval(be1) + eval(be2)
    },
    Expr::Mul(be1,be2) => { 
      eval(be1) * eval(be2)
    }
  }
}

fn eval_str1(s : &str) -> i64 {
  eval(&parse1(&lex(s)))
}

fn eval_str2(s : &str) -> i64 {
  eval(&parse2(&lex(s)))
}

#[derive(Debug,Copy,Clone,PartialEq)]
enum Token {
  Num(i64),
  Add,
  Mul,
  OpenParen,
  CloseParen
}

impl Token {
  fn precedence(&self) -> i32 {
    match self {
      Token::Add => 2,
      Token::Mul => 1,
      _ => panic!("not defined")
    }
  }
}

#[derive(Debug,Clone)]
enum Expr {
  Num(i64),
  Add(Box<Expr>,Box<Expr>),
  Mul(Box<Expr>,Box<Expr>)
}
