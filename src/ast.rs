use alloc::{boxed::Box, vec::Vec, string::String};



pub trait Expr {
//     fn interpret(&self);
    fn repr(&self) -> String;
}

// pub struct Def {
//     name: str,
//     args: Vec<Expr>,
//     body: Expr,
// }

pub struct Addition {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}
impl Expr for Addition {
    fn repr(&self) -> String {
        format!("(+ {} {})", self.left.repr(), self.right.repr())
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
