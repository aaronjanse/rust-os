use alloc::{boxed::Box, string::String, vec::Vec};
use crate::interpreter::{TokenType, TokenType::*};
// use crate::println;
use LangValue::*;

#[derive(Debug)]
#[derive(Clone)]
pub enum LangValue {
    LangNumber(f64),
    LangString(String),
    LangPair {left: Box<LangValue>, right: Box<LangValue>},
    LangNone,
}
impl Expr for LangValue {
    fn eval(&self) -> LangValue {
        self.clone()
    }
}
impl Representable for LangValue {
    fn repr(&self) -> String {
        format!("{:?}", self)
    }
}

pub enum ExprOrDecl {
    Expression(Box<dyn Expr>),
    Declaration(Decl),
}

// pub fn repr_lang_val(val: LangValue) -> String {
//     unimplemented!()
// }


pub trait Expr: Representable {
    fn eval(&self) -> LangValue;
}
pub trait Representable {
    fn repr(&self) -> String;
}

pub struct Decl {
    pub name: String,
    pub params: Vec<String>,
    pub body: Box<dyn Expr>,
}
impl Representable for Decl {
    fn repr(&self) -> String {
        format!("def {} ({:?}) -> {}", self.name, self.params, self.body.repr())
    }
}

pub struct FnCall {
    pub name: String,
    pub args: Vec<Box<dyn Expr>>,
}
impl Expr for FnCall {
    fn eval(&self) -> LangValue {
        LangNone
    }
}
impl Representable for FnCall {
    fn repr(&self) -> String {
        let mut arg_strs: Vec<String> = Vec::new();
        for i in 0..self.args.len() {
            arg_strs.push(self.args[i].repr());
        }
        format!("(do {} {:?})", self.name, arg_strs)
    }
}

pub struct Identifier {
    pub name: String,
}
impl Representable for Identifier {
    fn repr(&self) -> String {
        self.name.clone()
    }
}
impl Expr for Identifier {
    fn eval(&self) -> LangValue {
        LangNone
    }
}

pub struct BinaryExpr {
    pub oper: TokenType,
    pub left: Box<dyn Expr>,
    pub right: Box<dyn Expr>,
}
impl Representable for BinaryExpr {
    fn repr(&self) -> String {
        let oper_str = match self.oper {
            Star => "*",
            Slash => "/",
            Plus => "+",
            Minus => "-",
            _ => "?",
        };
        format!("({} {} {})", oper_str, self.left.repr(), self.right.repr())
    }
}
impl Expr for BinaryExpr {
    fn eval(&self) -> LangValue {
        match self.oper {
            Star | Slash | Plus | Minus => {
                let left_eval = self.left.eval();
                let left = match left_eval {
                    LangNumber(x) => x,
                    _ => panic!("NaN {:?}", left_eval),
                };
                let right_eval = self.right.eval();
                let right = match right_eval {
                    LangNumber(x) => x,
                    _ => panic!("NaN {:?}", right_eval),
                };

                LangNumber(match self.oper {
                    Plus => left + right,
                    Minus => left - right,
                    Star => left * right,
                    Slash => left / right,
                    _ => panic!("Cannot handle {:?}", self.oper),
                })
            },
            _ => panic!("Cannot eval {:?}", self.oper),
        }
    }
}

pub struct List {
    pub left: Box<dyn Expr>,
    pub right: Box<dyn Expr>,
}
impl Representable for List {
    fn repr(&self) -> String {
        format!("[{} {}]", self.left.repr(), self.right.repr())
    }
}
impl Expr for List {
    fn eval(&self) -> LangValue {
        LangPair{
            left: Box::new(self.left.eval()),
            right: Box::new(self.right.eval()),
        }
    }
}
