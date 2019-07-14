// pub trait Parser {
//     fn visit(&mut self, v: &mut dyn Visitor) -> VResult;
// }

use alloc::{boxed::Box, vec::Vec, string::String};
use crate::interpreter::{Token, TokenType, TokenType::*};
use crate::ast::*;
use crate::println;
use core::slice::Iter;
use core::iter::Peekable;


pub fn parseFile<'a>(mut tokens: &mut Peekable<Iter<Token>>) -> Box<Expr> {
    // root is a list of definitions

    return parseExpr(&mut tokens);
}

fn parseExpr<'a>(mut tokens: &mut Peekable<Iter<Token>>) -> Box<Expr> {
    return parseAddition(&mut tokens);
}

fn parseAddition<'a>(mut tokens: &mut Peekable<Iter<Token>>) -> Box<Expr> {
    return binaryParser(tokens, parseMult, &[Plus, Minus])
}

fn binaryParser<'a>(
            mut tokens: &mut Peekable<Iter<Token>>,
            subParser: fn(&mut Peekable<Iter<Token>>) -> Box<Expr>,
            opers: &[TokenType]) -> Box<Expr> {

    let mut expr = subParser(&mut tokens);

    loop {
        match tokens.peek() {
            Some(i) => {
                let mut found = false;
                for op in opers.iter() {
                    if i.kind == *op {
                        found = true;
                        let oper = tokens.next().expect("EOF!").kind;
                        let right = subParser(&mut tokens);
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

fn parseMult<'a>(mut tokens: &mut Peekable<Iter<Token>>) -> Box<Expr> {
    return binaryParser(tokens, parseUnary, &[Star, Slash])
}

fn parseUnary(mut tokens: &mut Peekable<Iter<Token>>) -> Box<Expr> {
    let tok = tokens.next().expect("unexpected EOF!");
    assert_eq!(tok.kind, Number);
    return Box::new(LiteralNumber{
        value: tok.literal.parse::<f64>().unwrap(),
    });
}
