// pub trait Parser {
//     fn visit(&mut self, v: &mut dyn Visitor) -> VResult;
// }

use alloc::{boxed::Box, vec::Vec, string::{String, ToString}};
use crate::interpreter::{Token, TokenType, TokenType::*};
use crate::ast::*;
use crate::ast::LangValue::*;
use core::slice::Iter;
use core::iter::Peekable;


pub fn parse_file(mut tokens: &mut Peekable<Iter<Token>>) -> ExprOrDecl {
    // root is a list of definitions

    return parse_expr_or_decl(&mut tokens);
}

fn parse_expr_or_decl(tokens: &mut Peekable<Iter<Token>>) -> ExprOrDecl {
    // we know it's a function if it's only LiteralIdentifiers followed by an equals sign
    let mut is_decl = true;
    let toks: Vec<Token> = tokens.cloned().collect();
    let mut i = 0;
    loop {
        match toks[i].kind {
            LiteralIdentifier => (),
            Equal => break,
            _ => {
                is_decl = false;
                break;
            },
        };
        i+=1;
    }

    let mut new_tokens = toks.iter().peekable();

    if is_decl {
        return ExprOrDecl::Declaration(parse_declaration(&mut new_tokens));
    } else {
        return ExprOrDecl::Expression(parse_statement(&mut new_tokens));
    }
}

fn parse_declaration(tokens: &mut Peekable<Iter<Token>>) -> Decl {
    let name = tokens.next().expect(" (de)").literal.clone();
    let mut params: Vec<String> = Vec::new();

    loop {
        match tokens.next() {
            Some(tok) => match tok.kind {
                Equal => break,
                LiteralIdentifier => params.push(tok.literal.clone()),
                _ => panic!("Unexpected {:?}", tok),
            },
            None => panic!("EOF (den)"),
        }
    };

    let body = parse_expr(tokens);

    Decl{
        name, params, body,
    }
}

fn parse_statement(mut tokens: &mut Peekable<Iter<Token>>) -> Box<dyn Expr> {
    return parse_fn_call(&mut tokens);
}

fn parse_fn_call(mut tokens: &mut Peekable<Iter<Token>>) -> Box<dyn Expr> {
    if tokens.peek().expect("EOF (fn?)").kind != LiteralIdentifier {
        return parse_expr(&mut tokens);
    }

    let toks: Vec<Token> = tokens.cloned().collect();
    let name = toks[0].literal.clone();
    
    let mut new_tokens = toks.iter().peekable();

    match toks[1].kind {
        LiteralIdentifier | LiteralString | Number |
        LeftParen | LeftCurlyBrace | LeftSquareBrace => {
            let mut args: Vec<Box<dyn Expr>> = Vec::new();
            new_tokens.next();
            loop {
                let expr = parse_expr(&mut new_tokens);
                args.push(expr);

                if new_tokens.peek().expect("Expecting semicolon.").kind == Semicolon {
                    new_tokens.next();
                    return Box::new(FnCall{name, args});
                }
            }
            
        },
        _ => {
            return parse_expr(&mut new_tokens);
        },
    }
}


fn parse_expr(tokens: &mut Peekable<Iter<Token>>) -> Box<dyn Expr> {
    return parse_addition(tokens)
}

fn parse_addition(tokens: &mut Peekable<Iter<Token>>) -> Box<dyn Expr> {
    return binary_parser(tokens, parse_mult, &[Plus, Minus])
}

fn parse_mult(tokens: &mut Peekable<Iter<Token>>) -> Box<dyn Expr> {
    return binary_parser(tokens, parse_unary, &[Star, Slash])
}

fn parse_unary(tokens: &mut Peekable<Iter<Token>>) -> Box<dyn Expr> {
    // FIXME: implement unary!
    return parse_single_token(tokens.next().expect("EOF!"));
}

fn parse_single_token(token: &Token) -> Box<dyn Expr> {
    match token.kind {
        LiteralIdentifier =>
            Box::new(Identifier{name: token.literal.clone()}) as Box<dyn Expr>,
        Number => 
            Box::new(LangNumber(
                token.literal.clone().parse::<f64>().unwrap()
            )) as Box<dyn Expr>,
        LiteralString =>
            Box::new(LangString(
                token.literal.clone()[1..token.literal.len()-1].to_string()
                )) as Box<dyn Expr>,
        _ => panic!("Unable to parse token {:?}", token),
    }
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
                        let oper = tokens.next().expect("EOF (bin)").kind;
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
