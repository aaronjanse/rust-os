use crate::ast;
use crate::interpret::{Evaluatable, Environment};
use alloc::{boxed::Box, string::String};
use core::fmt;

#[derive(Clone)]
pub enum LangValue {
    LangNumber(f64),
    LangString(String),
    LangFunc(ast::FuncEnv),
    LangPair {left: Box<LangValue>, right: Box<LangValue>},
    LangNone,
}
impl fmt::Display for LangValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use LangValue::*;
        match self {
            LangNumber(x) => write!(f, "{}", x),
            LangString(x) => write!(f, "{}", x),
            _ => write!(f, "<unknown val>"),
        }
    }
}
impl Evaluatable for LangValue {
    fn eval(&self, env: &Environment) -> LangValue {
        self.clone()
    }
}
impl ast::Expr for LangValue {}


impl Evaluatable for ast::List {
    fn eval(&self, env: &Environment) -> LangValue {
        LangValue::LangPair{
            left: Box::new(self.left.eval(&env)),
            right: Box::new(self.right.eval(&env)),
        }
    }
}

#[derive(Clone)]
pub struct Lambda {
    arg_destructure: Box<dyn ast::Destructure>,
    ret: Box<dyn ast::Expr>,
}
impl fmt::Display for Lambda {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(\\{} -> {})", self.arg_destructure, self.ret)
    }
}

// /*
// LangFunc


// \(x y z) ->
//     x + y*z


// \x' -> {
//     x = x';
//     \y' -> {
//         y = y';
//         \z -> {
//             x + y*z
//         }
//     }
// }

// */
