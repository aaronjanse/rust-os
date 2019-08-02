use alloc::{string::String, collections::btree_map::BTreeMap};
use crate::value::LangValue;
use crate::ast::*;

pub type Environment = BTreeMap<String, LangValue>;

pub trait Executable {
    fn exec(&self, mut env: &mut Environment);
}
pub trait Evaluatable {
    fn eval(&self, env: &Environment) -> LangValue;
}

impl Evaluatable for Scope {
  fn eval(&self, env: &Environment) -> LangValue {
    let mut tmp_env = env.clone();
    for i in 0..self.lines.len() {
      let ret = match &self.lines[i] {
        DeclOrExpr::Declaration(decl) => {
          decl.exec(&mut tmp_env);
          LangValue::LangNone
        },
        DeclOrExpr::Expression(expr) => {
          expr.eval(&tmp_env)
        }
      };
      if i == self.lines.len()-1 {
        return ret;
      }
    };
    LangValue::LangNone
  }
}

impl Executable for Decl {
  fn exec(&self, mut env: &mut Environment) {
    let val = self.right.eval(&env);
    self.left.destruct(&mut env, val)
  }
}
