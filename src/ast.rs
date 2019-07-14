use alloc::{boxed::Box, string::String};
use crate::interpreter::{TokenType, TokenType::*};
use crate::println;
use LangValue::*;

#[derive(Debug)]
pub enum LangValue {
    LangNumber(f64),
}

// pub fn repr_lang_val(val: LangValue) -> String {
//     unimplemented!()
// }


pub trait Expr {
    fn repr(&self) -> String;
    fn eval(&self) -> LangValue;
}

// pub struct Def {
//     name: str,
//     args: Vec<Expr>,
//     body: Expr,
// }

pub struct BinaryExpr {
    pub oper: TokenType,
    pub left: Box<dyn Expr>,
    pub right: Box<dyn Expr>,
}
impl Expr for BinaryExpr {
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

pub struct LiteralNumber {
    pub value: f64,
}
impl Expr for LiteralNumber {
    fn repr(&self) -> String {
        format!("{}", self.value)
    }
    fn eval(&self) -> LangValue {
        return LangNumber(self.value)
    }
}

// impl Expr for Addition {
//     fn interpret(&self) {
//         return self.left.interpret() + self.right.interpret();
//     }
// }
