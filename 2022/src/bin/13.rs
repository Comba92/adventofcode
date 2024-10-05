use std::{cmp::Ordering, iter::Peekable};

#[derive(Debug)]
enum Token { OPEN, CLOSE, NUMBER(String), COMMA }

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value { List(Vec<Value>), Number(u32) }
impl Value {
  fn new_list(values: &[Value]) -> Self {
    Value::List(Vec::from(values))
  }
}

impl Ord for Value {
  fn cmp(&self, other: &Self) -> Ordering {
    match (self, other) {
      (Value::Number(a), Value::Number(b)) => a.cmp(b),
      (Value::List(a), Value::List(b)) => {
        if a.is_empty() && b.is_empty() { return Ordering::Equal }
        if a.is_empty() { return Ordering::Less }
        if b.is_empty() { return Ordering::Greater }
  
        let mut i = 0;
        while i < a.len().min(b.len()) {
          let rec = &a[i].cmp(&b[i]);
          if *rec != Ordering::Equal { return *rec }
          i += 1;
        }
  
        a.len().cmp(&b.len())
      },
      (Value::Number(a), Value::List(_)) => {
        let new = Value::List(Vec::from([Value::Number(*a)]));
        new.cmp(other)
      }
      (Value::List(_), Value::Number(b)) => {
        let new = Value::List(Vec::from([Value::Number(*b)]));
        self.cmp(&new)
      }
    }
  }
}

impl PartialOrd for Value {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

fn scan(text: &str) -> Vec<Token> {
  let mut tokens = Vec::new();

  let mut iter = text.chars().peekable();
  while let Some(c) = iter.peek() {
    let token = match c {
      '[' => Token::OPEN,
      ']' => Token::CLOSE,
      ',' => Token::COMMA,
      c if c.is_digit(10) => {
        let num: String = iter
          .clone().take_while(|c| c.is_digit(10))
          .collect();

        Token::NUMBER(num)
      },
      _ => unreachable!()
    };

    match &token {
      Token::NUMBER(num) => { iter.nth(num.len()-1); } 
      _ => { iter.next(); } 
    }

    tokens.push(token);
  }

  tokens
}

fn parse<'a, I>(tokens: &mut Peekable<I>) -> Value 
where I: Iterator<Item = &'a Token>
{ 
  // skip opening bracket
  tokens.next();
  let mut values = Vec::new();

  while let Some(token) = tokens.peek() {
    match token {
      Token::OPEN => {
        values.push(parse(tokens));
      }
      Token::NUMBER(num) => {
        let num = num.parse().unwrap();
        values.push(Value::Number(num));
      }

      Token::CLOSE => { break; },
      Token::COMMA => {}
    }

    tokens.next();
  }

  Value::List(values)
}

fn main() {
  let input = include_str!("13.txt");
  let groups = input.split("\r\n\r\n");
  let mut values = Vec::new();

  for group in groups {
    for line in group.lines() {
      let tokens = scan(line);
      let value = parse(&mut tokens.iter().peekable());
      values.push(value);
    }
  }

  let mut result1 = 0;
  for (i, pair) in values.iter().as_slice().chunks(2).enumerate() {
    let first = &pair[0];
    let second = &pair[1];

    result1 += if first < second { i+1 } else { 0 };
  }

  let dividers = [
    Value::new_list(&[Value::new_list(&[Value::Number(2)])]),
    Value::new_list(&[Value::new_list(&[Value::Number(6)])]),
  ];

  values.append(&mut dividers.to_vec());  
  values.sort();

  let result2: usize = values.iter().enumerate()
  .filter(|(_, val)| dividers.contains(val))
  .map(|(i, _)| i+1)
  .product();

  println!("{result1} {result2}");
}