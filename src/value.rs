use crate::ast;
use crate::interpret::{Evaluatable, Environment};
use alloc::{boxed::Box, string::String};
use core::fmt;

#[derive(Debug)]
#[derive(Clone)]
pub enum LangValue {
    LangNumber(f64),
    LangString(String),
    LangPair {left: Box<LangValue>, right: Box<LangValue>},
    // LangFunc {arg_name: String},
    LangNone,
}
impl fmt::Display for LangValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use LangValue::*;
        match self {
            LangNumber(x) => write!(f, "{:?}", x),
            LangString(x) => write!(f, "{:?}", x),
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

// pub struct LangFuncData {
//     arg_name: String,
//     env: Box<Environment>,
//     ret: LangValue,
// }

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
