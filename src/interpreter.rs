use alloc::{vec::Vec, string::String};
use crate::println;

pub fn test_interpreter() {
  let text = String::from("true");
  let mut tokens: Vec<Token> = Vec::new();
  ParserIter::init(&text).parse(&mut tokens);
  println!("{:?}", tokens);
}

struct ParserIter<'a> {
  source: &'a String,
  iter: core::iter::Peekable<core::str::Chars<'a>>,
  next: Option<char>,
  buffer: String,
  line: usize,
}

impl<'a> ParserIter<'a> {
  fn init(source: &'a String) -> ParserIter<'a> {
    let mut iter = source.chars().peekable();
    let next = iter.next();
    ParserIter {
      source, iter, next,
      buffer: String::from(""),
      line: 0,
    }
  }

  fn parse(&mut self, tokens: &mut Vec<Token>) {
    let mut error: Option<String> = None;

    use TokenType::*;
    loop {
      let tok = match self.next() {
        None => Eof,
        Some(c) => match c {
          '\n' | '\r' | '\t' | ' ' => Ignore,

          '(' => LeftParen,
          ')' => RightParen,
          '[' => LeftSquareBrace,
          ']' => RightSquareBrace,
          '{' => LeftCurlyBrace,
          '}' => RightCurlyBrace,
          ':' => Colon,
          '+' => Plus,

          '!' => match self.peek() {
            Some('=') => {self.advance(); NotEqual},
            _ => Unrecognized,
          },
          '=' => match self.peek() {
            Some('=') => {self.advance(); EqualEqual},
            _ => Equal,
          },
          '|' => match self.peek() {
            Some('|') => {self.advance(); Or},
            Some('>') => {self.advance(); PipeForwards},
            _ => Pipe,
          },
          '-' => match self.peek() {
            Some('>') => {self.advance(); Arrow},
            Some('-') => {
              loop {
                match self.next() {
                  None => break,
                  Some('\n') => break,
                  _ => (),
                };
              }
              Comment
            }
            _ => Minus,
          },
          '<' => match self.peek() {
            Some('=') => {self.advance(); LessEq},
            _ => Less,
          }
          '>' => match self.peek() {
            Some('=') => {self.advance(); GreaterEq},
            _ => Greater,
          },

          '0'..='9' => {
            loop {
              match self.peek() {
                None => break,
                Some(c) => match c {
                  '0'..='9' => self.advance(),
                  '.' => {
                    self.advance();
                    loop {
                      match self.peek() {
                        None => break,
                        Some(c) => match c {
                          '0'..='9' => self.advance(),
                          _ => {
                            error = Some(format!("Invalid char {:?}", c));
                            break
                          },
                        }
                      }
                    }
                    break
                  },
                  _ => {
                    error = Some(format!("Invalid char {:?}", c));
                    break
                  },
                }
              }
            };
            Number
          },

          _ => Unrecognized,
        },
      };
      match error {
        None => {
          match tok {
            Eof => break,
            Ignore => self.buffer = String::from(""),
            _ => self.add_token(tokens, tok),
          }
        },
        Some(err) => {
          println!("Error on line {}:", self.line);
          let lines: Vec<&str> = self.source.lines().collect();
          println!("{}", lines[self.line]);
          println!("\n{}", err);
          break
        },
      }
    }
  }

  fn add_token(&mut self, tokens: &mut Vec<Token>, kind: TokenType) {
    tokens.push(Token {
      kind,
      line: self.line,
      literal: self.buffer.clone(),
    });
    self.buffer = String::from("")
  }

  fn next(&mut self) -> Option<char> {
    let cur = self.next;
    match cur {
      None => None,
      Some(c) => {
        self.buffer.push(c);
        if c == '\n' {
          self.line += 1;
        }
        self.next = self.iter.next();
        Some(c)
      }
    }
  }

  fn advance(&mut self) {
    self.next();
  }

  fn peek(&self) -> Option<char> {
    self.next
  }

  fn peek2(&mut self) -> Option<char> {
    match self.iter.peek() {
      Some(i) => Some(*i),
      None => None,
    }
  }
}


#[derive(Debug)]
struct Token {
  kind: TokenType,
  line: usize,
  literal: String,
}

#[allow(dead_code)]
#[derive(Debug)]
enum TokenType {
  Unrecognized,

  // Single-character tokens.
  LeftParen, RightParen,
  LeftSquareBrace, RightSquareBrace,
  LeftCurlyBrace, RightCurlyBrace,
  Colon, Pipe, Backslash,
  Plus, Minus, Star, Slash,
  Dollar,

  // One or two character tokens.
  Equal, EqualEqual, NotEqual,
  And, Or,
  Greater, GreaterEq,
  Less, LessEq,
  Arrow, // ->
  PipeForwards,  // |>

  // Literals.
  Identifier, Str, Number,

  // Keywords.
  True, False,
  If, Then, Else,
  Let, In,
  Yield,

  Comment,
  Ignore,
  Eof,
}