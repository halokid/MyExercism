use std::ops::{IndexMut, Mul};
use std::str::FromStr;

pub type Value = i32;

pub type Result<T> = std::result::Result<T, Error>;

pub type ForthResult = Result<()>;

#[derive(Clone, Debug)]
enum Instruction {
  Add,
  Sub,
  Mul,
  Div,
  Dup,
  Swap,
  Drop,
  Over,
  Number(Value),
  Call(Value),
}

struct Definition {
  name:   String,
  body:   Vec<Instruction>,
}

#[derive(Default)]
pub struct Forth {
  dict: Vec<Definition>,
  stack:  Vec<Value>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
  DivisionByZero,
  StackUnderflow,
  UnknownWord,
  InvalidWord,
}

fn parse_buildin(word: &str) -> Result<Instruction> {
  match word {
    "+" => Ok(Instruction::Add),
    "-" => Ok(Instruction::Sub),
    "*" => Ok(Instruction::Mul),
    "/" => Ok(Instruction::Div),
    "DUP" => Ok(Instruction::Dup),
    "SWAP" => Ok(Instruction::Swap),
    "DROP" => Ok(Instruction::Drop),
    "OVER" => Ok(Instruction::Over),
    _ => if let Ok(num) = Value::from_str(word) {
      Ok(Instruction::Number(num))
    } else {
      eprintln!("parse_buildin faild");
      Err(Error::UnknownWord)
    }
  }
}

impl Forth {
  fn parse_word<'a>(&mut self, word: &'a str,
                    remaining_input: &mut impl Iterator<Item = &'a str>) -> ForthResult {
    if word == ":" {
      self.parse_definition(remaining_input)
    } else {
      let instr = self.parse_normal_word(word)?;
      self.eval_instruction(instr)
    }
  }

 fn eval_instruction(&mut self, instr: Instruction) -> ForthResult {
  match instr {
    Instruction::Add => self.arith(|a, b| Ok(a + b)),
    Instruction::Sub => self.arith(|a, b| Ok(a - b)),
    Instruction::Mul => self.arith(|a, b| Ok(a * b)),
    Instruction::Div => self.arith(|a, b| if b == 0 {
      eprintln!("Division by zero");
      Err(Error::DivisionByZero)
    } else {
      Ok(a / b)
    }),
    // Instruction::Dup => self

  }
 }

  fn arith<F: FnOnce(Value, Value) -> Result<Value>>(&mut self, op: F) -> ForthResult {
    let rhs = self.pop()?;
    let lhs = self.pop()?;
    self.push(op(lhs, rhs)?);
    Ok(())
  }

  fn pop(&mut self) -> Result<Value> {
    if let Some(v) = self.stack.pop() {
      Ok(v)
    } else {
      eprintln!("Stack underflow!");
      Err(Error::StackUnderflow)
    }
  }

  fn push(&mut self, val: Value)  {
    self.stack.push(val);
  }


  fn parse_normal_word(&mut self, word: &str) -> Result<Instruction> {
    if word == ":" || word == ";" {
      Err(Error::InvalidWord)
    } else {
      let canonical = word.to_ascii_lowercase();
      if let Some(call) = self.find_defn(&canonical) {
        Ok(call)
      } else {
        parse_buildin(&canonical)
      }
    }
  }

  fn find_defn(&self, word: &str) -> Option<Instruction> {
    for (idx, defn) in self.dict.iter().enumerate().rev() {
      if defn.name == word {
        return  Some(Instruction::Call(idx.try_into().unwrap()));
      }
    }
    None
  }

  fn parse_definition<'a>(&mut self, iter: &mut impl Iterator<Item = &'a str>) -> ForthResult {
  }

  pub fn new() -> Forth {
    todo!()
  }

  pub fn stack(&self) -> &[Value] {
    todo!()
  }

  pub fn eval(&mut self, input: &str) -> Result {
    todo!("result of evaluating '{input}'")
  }
}

