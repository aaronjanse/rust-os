/*
parse_file
  parse_declaration
    parse_var_assign
    parse_fn_assign

parse_block
  parse_declaration
  parse_expression

parse_var_assign
  parse_destructure
  parse_expression
  parse_fn_call

parse_fn_assign
  parse_identifier
  parse_destructur
    
parse_destructure
  parse_identifier
  parse_list_destruct
    parse_list

parse_expression
*/

use alloc::{boxed::Box, vec::Vec, string::{ToString}};
use core::result::{Result, Result::{Ok, Err}};
use crate::scan::{Token, TokenType, TokenType::*};
use crate::ast::*;
use crate::value::LangValue::*;

type BoxedParserRes<T> = Result<Box<T>, &'static str>;
type ParserRes<T> = Result<T, &'static str>;


pub fn parse_file(mut tokens: &mut TokenIter) -> ParserRes<Vec<Decl>> {
  Ok(series_parse::<Decl>(&mut tokens, parse_declaration, &TokenType::Eof))
}

fn parse_declaration(mut tokens: &mut TokenIter) -> ParserRes<Decl> {
  try_parsers::<Decl>(&mut tokens, vec![
    parse_var_assign,
    parse_fn_assign,
  ])
}

fn parse_var_assign(mut tokens: &mut TokenIter) -> ParserRes<Decl> {
  let left = parse_destructure(&mut tokens)?;
  let right = parse_assign_value(&mut tokens)?;
  Ok(Decl{left, right})
}

fn parse_fn_assign(mut tokens: &mut TokenIter) -> ParserRes<Decl> {
  let name = parse_identifier(&mut tokens)?;
  let args = series_parse::<Box<dyn Destructure >>(
      &mut tokens, parse_destructure, &TokenType::Equal);
  if args.len() == 0 {
    return Err("No arguments.");
  }
  let body = parse_assign_value(&mut tokens)?;
  Ok(Decl{
    left: Box::new(name) as Box<dyn Destructure>,
    right: Box::new(FuncDef{args, body}) as Box<dyn Expr>,
  })
}

fn parse_assign_value(mut tokens: &mut TokenIter) -> BoxedParserRes<dyn Expr> {
  try_parsers::<Box<dyn Expr>>(&mut tokens, vec![
    parse_fn_call,
    parse_expression,
  ])
}

fn parse_expression(mut tokens: &mut TokenIter) -> BoxedParserRes<dyn Expr> {
  try_parsers::<Box<dyn Expr>>(&mut tokens, vec![
    parse_infix_arithmetic,
    parse_expression_nonbinary,
  ])
}

fn parse_expression_nonbinary(mut tokens: &mut TokenIter) -> BoxedParserRes<dyn Expr> {
  try_parsers::<Box<dyn Expr>>(&mut tokens, vec![
    parse_parens,
    parse_block,
    parse_expr_list,
    parse_single_token,
  ])
}

fn parse_parens(mut tokens: &mut TokenIter) -> BoxedParserRes<dyn Expr> {
  tokens.expect(LeftParen)?;
  let out = try_parsers::<Box<dyn Expr>>(&mut tokens, vec![
    parse_fn_call,
    parse_expression,
  ])?;
  tokens.expect(RightParen)?;
  Ok(out)
}

fn parse_block(mut tokens: &mut TokenIter) -> BoxedParserRes<dyn Expr> {
  tokens.expect(LeftCurlyBrace)?;
  let lines = series_parse::<DeclOrExpr>(
    &mut tokens, parse_decl_or_expr, &TokenType::RightCurlyBrace);
  tokens.expect(RightCurlyBrace)?;
  Ok(Box::new(Block{lines}) as Box<dyn Expr>)
}

fn parse_decl_or_expr(mut tokens: &mut TokenIter) -> ParserRes<DeclOrExpr> {
  tokens.bookmark();
  match parse_declaration(&mut tokens) {
    Ok(decl) => Ok(DeclOrExpr::Declaration(decl)),
    Err(_) => {
      tokens.revert();
      match parse_expression(&mut tokens) {
        Ok(expr) => Ok(DeclOrExpr::Expression(expr)),
        Err(_) => Err("Could parse neither decl nor expr."),
      }
    }
  }
}

fn parse_destructure(mut tokens: &mut TokenIter) -> BoxedParserRes<dyn Destructure> {
  Ok(Box::new(parse_identifier(&mut tokens)?) as Box<dyn Destructure>)
}

fn parse_fn_call(mut tokens: &mut TokenIter) -> BoxedParserRes<dyn Expr> {
  let func = parse_expression(&mut tokens)?;
  let arg = parse_expression(&mut tokens)?;
  Ok(Box::new(FuncCall{func, arg}) as Box<dyn Expr>)
}

fn parse_expr_list(mut tokens: &mut TokenIter) -> BoxedParserRes<dyn Expr> {
  parse_list(&mut tokens, parse_expression)
}


fn parse_list(
    mut tokens: &mut TokenIter,
    subparser: fn(&mut TokenIter) -> BoxedParserRes<dyn Expr>,
) -> BoxedParserRes<dyn Expr> {

    if !tokens.matches(LeftSquareBrace) {
        return Err("Expected [");
    }

    tokens.next();

    let mut items: Vec<Box<dyn Expr>> = Vec::new();
    items.push(subparser(&mut tokens)?);
    loop {
        match tokens.next().kind {
            Comma => {
                items.push(subparser(&mut tokens)?)
            },
            RightSquareBrace => {
                let mut out = Box::new(LangNone) as Box<dyn Expr>;
                while items.len() > 0 {
                    out = Box::new(List{
                        left: items.pop().expect("end?!"),
                        right: out,
                    });
                }
                return Ok(out);
            },
            _ => return Err("Expecting , or ]"),
        }
    }
}

fn parse_single_token(tokens: &mut TokenIter) -> BoxedParserRes<dyn Expr> {
  let token = tokens.peek();
  let out = match token.kind {
    LiteralIdentifier =>
      Ok(Box::new(Identifier{name: token.literal.clone()}) as Box<dyn Expr>),
    LiteralNumber => 
      Ok(Box::new(LangNumber(
        token.literal.clone().parse::<f64>().unwrap()
      )) as Box<dyn Expr>),
    LiteralString =>
      Ok(Box::new(LangString(
        token.literal.clone()[1..token.literal.len()-1].to_string()
      )) as Box<dyn Expr>),
    _ => return Err("Expecting single token."),
  };
  tokens.next();
  out
}

fn parse_identifier(tokens: &mut TokenIter) -> ParserRes<Identifier> {
    let token = tokens.next();
    match token.kind {
        LiteralIdentifier =>
            Ok(Identifier{name: token.literal.clone()}),
        _ => Err("Expecting identifier."),
    }
}

fn parse_infix_arithmetic(tokens: &mut TokenIter) -> BoxedParserRes<dyn Expr> {
    let bin = |oper| binary_parser(&mut tokens, parse_expression_nonbinary, oper);
    match bin(&[TokenType::Star, TokenType::Slash]) {
      Ok(x) => Ok(x),
      Err(_) => match bin(&[TokenType::Plus, TokenType::Minus]) {
        Ok(x) => Ok(x),
        Err(_) => Err("Could not parse infix arithmetic."),
      },
    }
}

fn binary_parser(
            mut tokens: &mut TokenIter,
            sub_parser: fn(&mut TokenIter) -> BoxedParserRes<dyn Expr>,
            opers: &[TokenType]) -> BoxedParserRes<dyn Expr> {

  let mut expr = sub_parser(&mut tokens)?;

  loop {
    if tokens.at_end() {
        return Ok(expr);
    }

    let op_token = tokens.peek();

    let mut found = false;
    for op in opers.iter() {
      if op_token.kind == *op {
        found = true;
        let oper = tokens.next().kind;
        let right = sub_parser(&mut tokens)?;
        expr = Box::new(BinaryExpr{
          oper, left: expr, right,
        });
        break;
      }
    }
    if !found {
      return Ok(expr);
    }
  }
}

fn try_parsers<T>(tokens: &mut TokenIter, fns: Vec<fn(&mut TokenIter) -> ParserRes<T>>) -> ParserRes<T> {
  if fns.len() == 0 {
    Err("Could not find parser.")
  } else {
    tokens.bookmark();
    match fns.remove(0)(&mut tokens) {
      Ok(x) => Ok(x),
      Err(_) => {
        tokens.revert();
        try_parsers::<T>(&mut tokens, fns)
      }
    }
  }
}

fn series_parse<T>(tokens: &mut TokenIter, func: fn(&mut TokenIter) -> ParserRes<T>, end: &TokenType) -> Vec<T> {
  let mut vals: Vec<T> = Vec::new();
  loop {
    vals.push(func(&mut tokens)?);
    if tokens.matches(end) {
        break
    }
  }
  vals
}

pub struct TokenIter {
  tokens: Vec<Token>,
  index: usize,
  bookmark: usize,
}
impl TokenIter {
  pub fn from(vec: Vec<Token>) -> TokenIter {
    TokenIter {
      tokens: vec,
      index: 0,
      bookmark: 0,
    }
  }
  fn peek(&self) -> Token {
    if self.index >= self.tokens.len() {
      panic!("Unexpected EOF.");
    }
    self.tokens[self.index].clone()
  }
  fn next(&mut self) -> Token {
    let val = self.peek();
    self.index += 1;
    val
  }
  fn bookmark(&mut self) {
    self.bookmark = self.index;
  }
  fn revert(&mut self) {
    self.index = self.bookmark;
  }
  fn matches(&self, kind: TokenType) -> bool {
    let reality = self.peek().kind;
    reality == kind
  }
  fn expect(&mut self, kind: TokenType) -> Result<bool, &'static str> {
    if self.matches(kind) {
      self.index += 1;
      Ok(true)
    } else {
      Err("Expecting {:?}", kind);
    }
  }
  fn at_end(&self) -> bool {
    self.index >= self.tokens.len()
  }
  pub fn prev(&mut self) -> Token {
    self.tokens[self.index-1].clone()
  }
}
