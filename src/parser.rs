// pub trait Parser {
//     fn visit(&mut self, v: &mut dyn Visitor) -> VResult;
// }

use alloc::{boxed::Box};
use crate::interpreter::{Token, TokenType, TokenType::*};
use crate::ast::*;
// use crate::println;
use core::slice::Iter;
use core::iter::Peekable;


pub fn parse_file(mut tokens: &mut Peekable<Iter<Token>>) -> Box<dyn Expr> {
    // root is a list of definitions

    return parse_expr(&mut tokens);
}

fn parse_expr(mut tokens: &mut Peekable<Iter<Token>>) -> Box<dyn Expr> {
    return parse_addition(&mut tokens);
}

fn parse_addition(tokens: &mut Peekable<Iter<Token>>) -> Box<dyn Expr> {
    return binary_parser(tokens, parse_mult, &[Plus, Minus])
}

fn binary_parser(
            mut tokens: &mut Peekable<Iter<Token>>,
            sub_parser: fn(&mut Peekable<Iter<Token>>) -> Box<dyn Expr>,
            opers: &[TokenType]) -> Box<dyn Expr> {

    let mut expr = sub_parser(&mut tokens);

    loop {
        match tokens.peek() {
            Some(i) => {
                let mut found = false;
                for op in opers.iter() {
                    if i.kind == *op {
                        found = true;
                        let oper = tokens.next().expect("EOF!").kind;
                        let right = sub_parser(&mut tokens);
                        expr = Box::new(BinaryExpr{
                            oper, left: expr, right,
                        });
                        break;
                    }
                }
                if !found {
                    return expr;
                }
            }
            None => return expr,
        }
    }
}

fn parse_mult(tokens: &mut Peekable<Iter<Token>>) -> Box<dyn Expr> {
    return binary_parser(tokens, parse_unary, &[Star, Slash])
}

fn parse_unary(tokens: &mut Peekable<Iter<Token>>) -> Box<dyn Expr> {
    let tok = tokens.next().expect("unexpected EOF!");
    assert_eq!(tok.kind, Number);
    return Box::new(LiteralNumber{
        value: tok.literal.parse::<f64>().unwrap(),
    });
}
