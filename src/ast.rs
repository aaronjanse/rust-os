use crate::value::LangValue;
use crate::interpret::{Environment, Evaluatable};
use alloc::{boxed::Box, string::String, vec::Vec};
// use crate::interpreter::{TokenType, TokenType::*};
use core::fmt;

fn indent(s: String) -> String {
    let mut out: Vec<String> = Vec::new();
    for line in s.lines() {
        out.push(format!("  {}", line))
    }
    out.join("\n")
}

pub struct Scope {
    pub lines: Vec<DeclOrExpr>,
}
impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut line_strs: Vec<String> = Vec::new();
        for i in 0..self.lines.len() {
            line_strs.push(indent(format!("{}", self.lines[i])));
        }
        write!(f, "{{\n{}\n}}", line_strs.join("\n"))
    }
}

pub enum DeclOrExpr {
    Declaration(Decl),
    Expression(Box<dyn Expr>),
}
impl fmt::Display for DeclOrExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use DeclOrExpr::*;
        match self {
            Declaration(x) => write!(f, "{}", x),
            Expression(x) => write!(f, "{}", x),
        }
    }
}

// {left} = {right};
pub struct Decl {
    pub left: Box<dyn Destructure>,
    pub right: Box<dyn Expr>,
}
impl fmt::Display for Decl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} =\n{};", self.left, indent(format!("{}", self.right)))
    }
}

pub trait Destructure: fmt::Display {
    fn destruct(&self, env: &mut Environment, val: LangValue);
}

pub struct Identifier {
    name: String,
}
impl Destructure for Identifier {
    fn destruct(&self, env: &mut Environment, val: LangValue) {
        env.insert(self.name.clone(), val);
    }
}
impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}


pub trait Expr: fmt::Display + Evaluatable {}


// impl Expr for LangValue {
//     fn eval(&self) -> LangValue {
//         self.clone()
//     }
// }
// impl Representable for LangValue {
//     fn repr(&self) -> String {
//         format!("{:?}", self)
//     }
// }



// // these change the environment
// pub enum Assign {
//     DefineFunction
//     AssignVariable
// }

// // pub fn repr_lang_val(val: LangValue) -> String {
// //     unimplemented!()
// // }



// pub struct Decl {
//     pub name: String,
//     pub params: Vec<String>,
//     pub body: Box<dyn Expr>,
// }
// impl Representable for DefineFunction {
//     fn repr(&self) -> String {
//         format!("def {} ({:?}) -> {}", self.name, self.params, self.body.repr())
//     }
// }

// pub struct FnCall {
//     pub name: String,
//     pub args: Vec<Box<dyn Expr>>,
// }
// impl Expr for FnCall {
//     fn eval(&self) -> LangValue {
//         LangNone
//     }
// }
// impl Representable for FnCall {
//     fn repr(&self) -> String {
//         let mut arg_strs: Vec<String> = Vec::new();
//         for i in 0..self.args.len() {
//             arg_strs.push(self.args[i].repr());
//         }
//         format!("(do {} {:?})", self.name, arg_strs)
//     }
// }

// pub struct Identifier {
//     pub name: String,
// }
// impl Representable for Identifier {
//     fn repr(&self) -> String {
//         self.name.clone()
//     }
// }
// impl Expr for Identifier {
//     fn eval(&self) -> LangValue {
//         LangNone
//     }
// }

// pub struct BinaryExpr {
//     pub oper: TokenType,
//     pub left: Box<dyn Expr>,
//     pub right: Box<dyn Expr>,
// }
// impl Representable for BinaryExpr {
//     fn repr(&self) -> String {
//         let oper_str = match self.oper {
//             Star => "*",
//             Slash => "/",
//             Plus => "+",
//             Minus => "-",
//             _ => "?",
//         };
//         format!("({} {} {})", oper_str, self.left.repr(), self.right.repr())
//     }
// }
// impl Expr for BinaryExpr {
//     fn eval(&self) -> LangValue {
//         match self.oper {
//             Star | Slash | Plus | Minus => {
//                 let left_eval = self.left.eval();
//                 let left = match left_eval {
//                     LangNumber(x) => x,
//                     _ => panic!("NaN {:?}", left_eval),
//                 };
//                 let right_eval = self.right.eval();
//                 let right = match right_eval {
//                     LangNumber(x) => x,
//                     _ => panic!("NaN {:?}", right_eval),
//                 };

//                 LangNumber(match self.oper {
//                     Plus => left + right,
//                     Minus => left - right,
//                     Star => left * right,
//                     Slash => left / right,
//                     _ => panic!("Cannot handle {:?}", self.oper),
//                 })
//             },
//             _ => panic!("Cannot eval {:?}", self.oper),
//         }
//     }
// }

// pub struct List {
//     pub left: Box<dyn Expr>,
//     pub right: Box<dyn Expr>,
// }
// impl Representable for List {
//     fn repr(&self) -> String {
//         format!("[{} {}]", self.left.repr(), self.right.repr())
//     }
// }
// impl Expr for List {
//     fn eval(&self) -> LangValue {
//         LangPair{
//             left: Box::new(self.left.eval()),
//             right: Box::new(self.right.eval()),
//         }
//     }
// }

// pub struct Block {
//     pub items: Vec<ExprOrDecl>,
// }
// impl Representable for Block {
//     fn repr(&self) -> String {
//         let mut strs: Vec<String> = Vec::new();
//         let mut i = 0;
//         loop {
//             if i >= self.items.len() {
//                 break;
//             }
//             strs.push(self.items[i].repr());
//             i += 1;
//         }
//         format!("{{{}}}", strs.join("\n"))
//     }
// }
// impl Expr for Block {
//     fn eval(&self) -> LangValue {
//         LangNone
//     }
// }
