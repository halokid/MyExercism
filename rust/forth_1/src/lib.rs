use std::collections::HashMap;

pub type Value = i32;
pub type ForthResult = std::result::Result<(), Error>;

#[derive(Debug)]
pub struct Forth {
  stack:  Vec<Value>,
  dict_items: Vec<DictItem>,
  dict: HashMap<String, u32>,
}

#[derive(Clone, Debug)]
enum Term {
  Literal(Value),
  Call(u32),
}

#[derive(Clone, Debug)]
enum Op {
  Plus,
  Minus,
  Mul,
  Div,
  Dup,
  Drop,
  Swap,
  Over,
}

#[derive(Clone, Debug)]
enum DictItem {
  Op(Op),
  Terms(Vec<Term>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
  DivisionByZero,
  StackUnderflow,
  UnknownWord,
  InvalidWord,
}

use Op::{Div, Drop, Dup, Minus, Mul, Over, Plus, Swap};

impl Forth {
  pub fn new() -> Forth {
    let mut dict = HashMap::new();
    let mut dict_items = vec![];
    for (word, op) in ["+", "-", "*", "/", "dup", "drop", "swap", "over"].iter()
      .zip([Plus, Minus, Mul, Div, Dup, Drop, Swap, Over].iter()) {

      dict_items.push(DictItem::Op(op.clone()));

      dict.insert(
        word.to_lowercase(),
        (dict_items.len() - 1) as u32,
      );
    }

    Forth {
      stack: vec![],
      dict_items,
      dict,
    }
  }

  pub fn stack(&self) -> &[Value] {
    &self.stack
  }

  fn parse(&mut self, input: &str) -> Result<Vec<Term>, Error> {
    let mut iter = input.split_ascii_whitespace();
    self.do_parse(0, &mut iter)
  }

  pub fn eval(&mut self, input: &str) -> ForthResult {
    let v = self.parse(input)?;
    self.do_eval(v.iter())?;
    Ok(())
  }
}
