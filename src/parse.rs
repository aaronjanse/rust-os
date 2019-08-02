// // pub trait Parser {
// //     fn visit(&mut self, v: &mut dyn Visitor) -> VResult;
// // }

// use alloc::{boxed::Box, vec::Vec, string::{String, ToString}};
// use core::result::{Result, Result::{Ok, Err}};
// use crate::interpreter::{Token, TokenType, TokenType::*};
// use crate::ast::*;
// use crate::ast::LangValue::*;

// type BoxedParserRes<T> = Result<Box<T>, &'static str>;
// type ParserRes<T> = Result<T, &'static str>;


// pub fn parse_file(mut tokens: &mut TokenIter) -> ParserRes<Vec<Decl>> {
//     let mut decls: Vec<Decl> = Vec::new();
//     loop {
//         if tokens.at_end() {
//             return Ok(decls);
//         }

//         let decl = parse_declaration(&mut tokens)?;
//         decls.push(decl);
//     }
// }

// fn parse_expr_or_decl(mut tokens: &mut TokenIter) -> ParserRes<ExprOrDecl> {
//     // we know it's a function if it's only LiteralIdentifiers followed by an equals sign
//     tokens.bookmark();
//     let mut is_decl = true;
//     loop {
//         match tokens.next().kind {
//             LiteralIdentifier => (),
//             Equal => break,
//             _ => {
//                 is_decl = false;
//                 break;
//             },
//         };
//     }
//     tokens.revert();

//     if is_decl {
//         Ok(ExprOrDecl::Declaration(parse_declaration(&mut tokens)?))
//     } else {
//         Ok(ExprOrDecl::Expression(parse_statement(&mut tokens)?))
//     }
// }

// fn parse_assignment(mut tokens: &mut TokenIter) -> ParserRes<Decl> {
//     let name = tokens.next().literal.clone();
//     let mut params: Vec<String> = Vec::new();

//     loop {
//         let next = tokens.next();
//         match next.kind {
//             Equal => break,
//             LiteralIdentifier => params.push(next.literal.clone()),
//             _ => return Err("Expecting '=' or identifier."),
//         }
//     };

//     let body = parse_statement(&mut tokens)?;

//     Ok(Decl{
//         name, params, body,
//     })
// }

// fn parse_declaration(mut tokens: &mut TokenIter) -> ParserRes<Decl> {
//     let name = tokens.next().literal.clone();
//     let mut params: Vec<String> = Vec::new();

//     loop {
//         let next = tokens.next();
//         match next.kind {
//             Equal => break,
//             LiteralIdentifier => params.push(next.literal.clone()),
//             _ => return Err("Expecting '=' or identifier."),
//         }
//     };

//     let body = parse_statement(&mut tokens)?;

//     Ok(Decl{
//         name, params, body,
//     })
// }

// fn parse_statement(mut tokens: &mut TokenIter) -> BoxedParserRes<dyn Expr> {
//     match parse_fn_call(&mut tokens) {
//         Err(_) => parse_expr(&mut tokens),
//         Ok(x) => Ok(x),
//     }
// }

// fn parse_fn_call(mut tokens: &mut TokenIter) -> BoxedParserRes<dyn Expr> {
//     if !tokens.matches(LiteralIdentifier) {
//         return parse_expr(&mut tokens);
//     }

//     tokens.bookmark();

//     let name = tokens.next().literal.clone();
//     let mut args: Vec<Box<dyn Expr>> = Vec::new();

//     match tokens.peek().kind {
//         LiteralIdentifier | LiteralString | LiteralNumber |
//         LeftParen | LeftCurlyBrace | LeftSquareBrace => {
//             loop {
//                 let expr = parse_expr(&mut tokens)?;
//                 args.push(expr);

//                 match tokens.peek().kind {
//                     Semicolon => {
//                         tokens.next();
//                         break;
//                     },
//                     RightParen => {
//                         break;
//                     },
//                     _ => (),
//                 }
//             }
//         },
//         _ => {
//             tokens.revert();
//             return parse_expr(&mut tokens);
//         },
//     }

//     Ok(Box::new(FnCall{name, args}))
// }

// fn parse_expr(mut tokens: &mut TokenIter) -> BoxedParserRes<dyn Expr> {
//     parse_block(&mut tokens)
// }

// pub fn parse_block(mut tokens: &mut TokenIter) -> BoxedParserRes<dyn Expr> {
//     if !tokens.matches(LeftCurlyBrace) {
//         return parse_parens(&mut tokens);
//     }
//     tokens.next();

//     let mut items: Vec<ExprOrDecl> = Vec::new();
//     loop {
//         if tokens.matches(RightCurlyBrace) {
//             tokens.next();
//             return Ok(Box::new(Block{items}));
//         }
//         let expr_or_decl = parse_expr_or_decl(&mut tokens)?;
//         items.push(expr_or_decl);
//     }
// }


// fn parse_parens(mut tokens: &mut TokenIter) -> BoxedParserRes<dyn Expr> {
//     if !tokens.matches(LeftParen) {
//         if tokens.matches(LeftSquareBrace) {
//             return parse_list(&mut tokens, parse_expr);
//         } else {
//             return parse_addition(&mut tokens);
//         }
//     }

//     tokens.next();

//     let out = parse_fn_call(&mut tokens)?;

//     tokens.expect(RightParen);

//     Ok(out)
// }

// fn parse_list(
//     mut tokens: &mut TokenIter,
//     subparser: fn(&mut TokenIter) -> BoxedParserRes<dyn Expr>,
// ) -> BoxedParserRes<dyn Expr> {

//     if !tokens.matches(LeftSquareBrace) {
//         return Err("Expected [");
//     }

//     tokens.next();

//     let mut items: Vec<Box<dyn Expr>> = Vec::new();
//     items.push(subparser(&mut tokens)?);
//     loop {
//         match tokens.next().kind {
//             Comma => {
//                 items.push(parse_expr(&mut tokens)?)
//             },
//             RightSquareBrace => {
//                 let mut out = Box::new(LangNone) as Box<dyn Expr>;
//                 while items.len() > 0 {
//                     out = Box::new(List{
//                         left: items.pop().expect("end?!"),
//                         right: out,
//                     });
//                 }
//                 return Ok(out);
//             },
//             _ => return Err("Expecting , or ]"),
//         }
//     }
// }

// fn parse_addition(tokens: &mut TokenIter) -> BoxedParserRes<dyn Expr> {
//     binary_parser(tokens, parse_mult, &[Plus, Minus])
// }

// fn parse_mult(tokens: &mut TokenIter) -> BoxedParserRes<dyn Expr> {
//     binary_parser(tokens, parse_unary, &[Star, Slash])
// }

// fn parse_unary(tokens: &mut TokenIter) -> BoxedParserRes<dyn Expr> {
//     // FIXME: implement unary!
//     parse_single_token(tokens.next())
// }

// fn parse_single_token(token: Token) -> BoxedParserRes<dyn Expr> {
//     match token.kind {
//         LiteralIdentifier =>
//             Ok(Box::new(Identifier{name: token.literal.clone()}) as Box<dyn Expr>),
//         LiteralNumber => 
//             Ok(Box::new(LangNumber(
//                 token.literal.clone().parse::<f64>().unwrap()
//             )) as Box<dyn Expr>),
//         LiteralString =>
//             Ok(Box::new(LangString(
//                 token.literal.clone()[1..token.literal.len()-1].to_string()
//                 )) as Box<dyn Expr>),
//         _ => Err("Expecting single token."),
//     }
// }

// fn parse_identifier(tokens: &mut TokenIter)  -> BoxedParserRes<dyn Expr> {
//     let token = tokens.next();
//     match token.kind {
//         LiteralIdentifier =>
//             Ok(Box::new(Identifier{name: token.literal.clone()}) as Box<dyn Expr>),
//         _ => Err("Expecting identifier."),
//     }
// }

// fn binary_parser(
//             mut tokens: &mut TokenIter,
//             sub_parser: fn(&mut TokenIter) -> BoxedParserRes<dyn Expr>,
//             opers: &[TokenType]) -> BoxedParserRes<dyn Expr> {

//     let mut expr = sub_parser(&mut tokens)?;

//     loop {
//         if tokens.at_end() {
//             return Ok(expr);
//         }

//         let op_token = tokens.peek();

//         let mut found = false;
//         for op in opers.iter() {
//             if op_token.kind == *op {
//                 found = true;
//                 let oper = tokens.next().kind;
//                 let right = sub_parser(&mut tokens)?;
//                 expr = Box::new(BinaryExpr{
//                     oper, left: expr, right,
//                 });
//                 break;
//             }
//         }
//         if !found {
//             return Ok(expr);
//         }
//     }
// }

// pub struct TokenIter {
//     tokens: Vec<Token>,
//     index: usize,
//     bookmark: usize,
// }
// impl TokenIter {
//     pub fn from(vec: Vec<Token>) -> TokenIter {
//         TokenIter {
//             tokens: vec,
//             index: 0,
//             bookmark: 0,
//         }
//     }
//     fn peek(&self) -> Token {
//         if self.index >= self.tokens.len() {
//             panic!("Unexpected EOF.");
//         }
//         self.tokens[self.index].clone()
//     }
//     fn next(&mut self) -> Token {
//         let val = self.peek();
//         self.index += 1;
//         val
//     }
//     fn bookmark(&mut self) {
//         self.bookmark = self.index;
//     }
//     fn revert(&mut self) {
//         self.index = self.bookmark;
//     }
//     fn matches(&self, kind: TokenType) -> bool {
//         let reality = self.peek().kind;
//         reality == kind
//     }
//     fn expect(&mut self, kind: TokenType) {
//         if !self.matches(kind) {
//             panic!("Expecting {:?}", kind);
//         }
//         self.index += 1;
//     }
//     fn at_end(&self) -> bool {
//         self.index >= self.tokens.len()
//     }
//     pub fn prev(&mut self) -> Token {
//         self.tokens[self.index-1].clone()
//     }
// }
