use alloc::{boxed::Box, string::String};
use crate::interpreter::TokenType;



pub trait Expr {
//     fn interpret(&self);
    fn repr(&self) -> String;
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
            TokenType::Star => "*",
            TokenType::Slash => "/",
            TokenType::Plus => "+",
            TokenType::Minus => "-",
            _ => "?",
        };
        format!("({} {} {})", oper_str, self.left.repr(), self.right.repr())
    }
}

pub struct LiteralNumber {
    pub value: f64,
}
impl Expr for LiteralNumber {
    fn repr(&self) -> String {
        format!("{}", self.value)
    }
}

// impl Expr for Addition {
//     fn interpret(&self) {
//         return self.left.interpret() + self.right.interpret();
//     }
// }
