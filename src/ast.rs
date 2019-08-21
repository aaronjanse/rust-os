use crate::value::{LangValue, Lambda};
use crate::interpret::{Environment, Evaluatable};
use alloc::{boxed::Box, string::String, vec::Vec};
use crate::scan::{TokenType};
use core::fmt;

fn indent(s: String) -> String {
    let mut out: Vec<String> = Vec::new();
    for line in s.lines() {
        out.push(format!("  {}", line))
    }
    out.join("\n")
}

#[derive(Clone)]
pub struct Scope {
    pub env: Box<Environment>,
    pub lines: Vec<DeclOrExpr>,
}
impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut line_strs: Vec<String> = Vec::new();
        for i in 0..self.lines.len() {
            line_strs.push(indent(format!("{}", self.lines[i])));
        }
        write!(f, "{{<env>\n{}\n}}", line_strs.join("\n"))
    }
}

#[derive(Clone)]
pub struct Block {
    pub lines: Vec<DeclOrExpr>,
}
impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut line_strs: Vec<String> = Vec::new();
        for i in 0..self.lines.len() {
            line_strs.push(indent(format!("{}", self.lines[i])));
        }
        write!(f, "{{\n{}\n}}", line_strs.join("\n"))
    }
}

#[derive(Clone)]
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
#[derive(Clone)]
pub struct Decl {
    pub left: Box<dyn Destructure>,
    pub right: Box<dyn Expr>,
}
impl fmt::Display for Decl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} =\n{};", self.left, indent(format!("{}", self.right)))
    }
}

#[derive(Clone)]
pub struct FuncEnv {
  pub env: Environment,
  pub func: Lambda,
}
impl fmt::Display for FuncEnv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{<fn env>{}}}", self.func)
    }
}

#[derive(Clone)]
pub struct FuncDef {
    pub args: Vec<Box<dyn Destructure>>,
    pub body: Box<dyn Expr>,
}
impl fmt::Display for FuncDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arg_strs: Vec<String> = self.args.iter().map(|x| format!("{}", x)).collect();
        write!(f, "(\\({}) ->\n{};",
            arg_strs.join(" "),
            indent(format!("{}", self.body))
        )
    }
}

pub trait Destructure: fmt::Display + DestructureClone {
    fn destruct(&self, env: &mut Environment, val: LangValue);
}
trait DestructureClone {
    fn clone_box(&self) -> Box<dyn Destructure>;
}
impl<T> DestructureClone for T
where
    T: 'static + Destructure + Clone,
{
    fn clone_box(&self) -> Box<dyn Destructure> {
        Box::new(self.clone())
    }
}
impl Clone for Box<dyn Destructure> {
    fn clone(&self) -> Box<dyn Destructure> {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct Identifier {
    pub name: String,
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

pub trait Expr: fmt::Display + Evaluatable + ExprClone {}

trait ExprClone {
    fn clone_box(&self) -> Box<dyn Expr>;
}
impl<T> ExprClone for T
where
    T: 'static + Expr + Clone,
{
    fn clone_box(&self) -> Box<dyn Expr> {
        Box::new(self.clone())
    }
}
impl Clone for Box<dyn Expr> {
    fn clone(&self) -> Box<dyn Expr> {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct FuncCall {
    pub func: Box<dyn Expr>,
    pub arg: Box<dyn Expr>,
}
impl fmt::Display for FuncCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})", self.func, self.arg)
    }
}

#[derive(Clone)]
pub struct BinaryExpr {
    pub oper: TokenType,
    pub left: Box<dyn Expr>,
    pub right: Box<dyn Expr>,
}
impl fmt::Display for BinaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let oper_str = match self.oper {
            TokenType::Slash => "/",
            TokenType::Star => "*",
            TokenType::Plus => "+",
            TokenType::Minus => "-",
            _ => "?",
        };
        write!(f, "{}{}{}", self.left, oper_str, self.right)
    }
}
impl Expr for BinaryExpr {}

#[derive(Clone)]
pub struct List {
    pub left: Box<dyn Expr>,
    pub right: Box<dyn Expr>,
}
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{} {}]", self.left, self.right)
    }
}
impl Expr for List {}
