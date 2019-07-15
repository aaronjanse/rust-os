// pub trait Parser {
//     fn visit(&mut self, v: &mut dyn Visitor) -> VResult;
// }

use alloc::{boxed::Box, vec::Vec, string::{String, ToString}};
use crate::interpreter::{Token, TokenType, TokenType::*};
use crate::ast::*;
use crate::ast::LangValue::*;


pub fn parse_file(mut tokens: &mut TokenIter) -> ExprOrDecl {
    return parse_expr_or_decl(&mut tokens);
}

fn parse_expr_or_decl(mut tokens: &mut TokenIter) -> ExprOrDecl {
    // we know it's a function if it's only LiteralIdentifiers followed by an equals sign
    tokens.bookmark();
    let mut is_decl = true;
    loop {
        match tokens.next().kind {
            LiteralIdentifier => (),
            Equal => break,
            _ => {
                is_decl = false;
                break;
            },
        };
    }
    tokens.revert();

    if is_decl {
        return ExprOrDecl::Declaration(parse_declaration(&mut tokens));
    } else {
        return ExprOrDecl::Expression(parse_statement(&mut tokens));
    }
}

fn parse_declaration(mut tokens: &mut TokenIter) -> Decl {
    let name = tokens.next().literal.clone();
    let mut params: Vec<String> = Vec::new();

    loop {
        let next = tokens.next();
        match next.kind {
            Equal => break,
            LiteralIdentifier => params.push(next.literal.clone()),
            _ => panic!("Expected '=' or identifier, got {:?}", next),
        }
    };

    let body = parse_statement(&mut tokens);

    Decl{
        name, params, body,
    }
}

fn parse_statement(mut tokens: &mut TokenIter) -> Box<dyn Expr> {
    return parse_fn_call(&mut tokens);
}

fn parse_fn_call(mut tokens: &mut TokenIter) -> Box<dyn Expr> {
    if !tokens.matches(LiteralIdentifier) {
        return parse_expr(&mut tokens);
    }

    tokens.bookmark();

    let name = tokens.next().literal.clone();
    let mut args: Vec<Box<dyn Expr>> = Vec::new();

    match tokens.peek().kind {
        LiteralIdentifier | LiteralString | Number |
        LeftParen | LeftCurlyBrace | LeftSquareBrace => {
            loop {
                let expr = parse_expr(&mut tokens);
                args.push(expr);

                match tokens.peek().kind {
                    Semicolon => {
                        tokens.next();
                        break;
                    },
                    RightParen => {
                        break;
                    },
                    _ => (),
                }
            }
        },
        _ => {
            tokens.revert();
            return parse_expr(&mut tokens);
        },
    }

    return Box::new(FnCall{name, args});
}

fn parse_expr(mut tokens: &mut TokenIter) -> Box<dyn Expr> {
    return parse_square_braces(&mut tokens)
}

fn parse_square_braces(mut tokens: &mut TokenIter) -> Box<dyn Expr> {
    if !tokens.matches(LeftSquareBrace) {
        return parse_parens(&mut tokens)
    }

    tokens.next();

    let mut items: Vec<Box<dyn Expr>> = Vec::new();
    items.push(parse_expr(&mut tokens));
    loop {
        match tokens.next().kind {
            Comma => {
                items.push(parse_expr(&mut tokens))
            },
            RightSquareBrace => {
                let mut out = Box::new(LangNone) as Box<dyn Expr>;
                while items.len() > 0 {
                    out = Box::new(List{
                        left: items.pop().expect("end?!"),
                        right: out,
                    });
                }
                return out;
            },
            _ => panic!("Expecting , or ]"),
        }
    }
}

fn parse_parens(mut tokens: &mut TokenIter) -> Box<dyn Expr> {
    if !tokens.matches(LeftParen) {
        return parse_addition(&mut tokens);
    }

    tokens.next();

    let out = parse_fn_call(&mut tokens);

    tokens.expect(RightParen);

    out
}

fn parse_addition(tokens: &mut TokenIter) -> Box<dyn Expr> {
    return binary_parser(tokens, parse_mult, &[Plus, Minus])
}

fn parse_mult(tokens: &mut TokenIter) -> Box<dyn Expr> {
    return binary_parser(tokens, parse_unary, &[Star, Slash])
}

fn parse_unary(tokens: &mut TokenIter) -> Box<dyn Expr> {
    // FIXME: implement unary!
    return parse_single_token(tokens.next());
}

fn parse_single_token(token: Token) -> Box<dyn Expr> {
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
            mut tokens: &mut TokenIter,
            sub_parser: fn(&mut TokenIter) -> Box<dyn Expr>,
            opers: &[TokenType]) -> Box<dyn Expr> {

    let mut expr = sub_parser(&mut tokens);

    loop {
        if tokens.at_end() {
            return expr;
        }

        let op_token = tokens.peek();

        let mut found = false;
        for op in opers.iter() {
            if op_token.kind == *op {
                found = true;
                let oper = tokens.next().kind;
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
    fn expect(&mut self, kind: TokenType) {
        if !self.matches(kind) {
            panic!("Expecting {:?}", kind);
        }
        self.index += 1;
    }
    fn at_end(&self) -> bool {
        self.index >= self.tokens.len()
    }
}
