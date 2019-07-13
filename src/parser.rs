// pub trait Parser {
//     fn visit(&mut self, v: &mut dyn Visitor) -> VResult;
// }

use alloc::{boxed::Box, vec::Vec, string::String};
use crate::interpreter::{Token, TokenType::*};
use crate::ast::*;
use crate::println;


pub fn parseFile<'a>(tokens: &[Token]) -> Box<Expr> {
    // root is a list of definitions

    return parseAddition(tokens);
}

fn parseExpr<'a>(tokens: &[Token]) -> Box<Expr> {
    if tokens.len() == 1 {
        assert_eq!(tokens[0].kind, Number);
        return Box::new(LiteralNumber{
            value: tokens[0].literal.parse::<f64>().unwrap(),
        });
    }
    return parseAddition(tokens);
}

fn parseAddition<'a>(tokens: &[Token]) -> Box<Expr> {
    for (i, tok) in tokens.iter().enumerate() {
        if tok.kind == Plus {
            let left = parseExpr(&tokens[..i]);
            let right = parseExpr(&tokens[i+1..]);
            return Box::new(Addition{
                left: left,
                right: right,
            });
        }
    }
    panic!("Could not parse!")
}

// fn parseDefList(toks: Vec<Token>) {
//     assert_eq!(toks[0].kind, Identifier);

//     let defName = toks[0];

// }
