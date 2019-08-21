use alloc::{string::String, collections::btree_map::BTreeMap};
use crate::value::{LangValue, Lambda};
use crate::ast::*;
use crate::scan::TokenType;

pub type Environment = BTreeMap<String, LangValue>;

pub trait Executable {
    fn exec(&self, env: &mut Environment);
}
pub trait Evaluatable {
    fn eval(&self, env: &Environment) -> LangValue;
}

impl Evaluatable for Block {
  fn eval(&self, env: &Environment) -> LangValue {
    Scope{env: env.clone(), lines: self.lines}
  }
}
impl Expr for Block {}

impl Evaluatable for Scope {
  fn eval(&self, env: &Environment) -> LangValue {
    let mut tmp_env = self.env.clone();
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

impl Evaluatable for FuncEnv {
  fn eval(&self, env: &Environment) -> LangValue {
    self
  }
}

impl Evaluatable for FuncDef {
  fn eval(&self, env: &Environment) -> LangValue {
    let out: dyn Expr;
    let first = true;
    for arg in self.args {
      let body: dyn Evaluatable = if first {
        first = false;
        self.body
      } else {
        out
      };
      out = Lambda{
        arg_destructure: arg,
        ret: body,
      }
    }
    FuncEnv{
      env: env.clone(),
      func: out,
    }
  }
}
impl Expr for FuncDef {}

impl Evaluatable for FuncCall {
  fn eval(&self, env: &Environment) -> LangValue {
    let func_env: dyn Evaluatable = self.func.eval(&env);
    match func_env {
      FuncEnv => (),
      _ => panic!("Cannot call function without an environment:\n{}", func_env),
    };

    let arg_val = self.arg.eval(&env);
    let tmpenv = func_env.env.clone();
    let lambda = func_env.func;
    lambda.arg_destructure.destruct(&mut tmpenv, arg_val);

    let val = lambda.ret.eval(&tmpenv); 
    match val {
      LangValue::LangFunc(_) => LangFunc(FuncEnv{
        env: tmpenv,
        func: val,
      },
      _ => val,
    }
  }
}
impl Expr for FuncCall {}

impl Evaluatable for BinaryExpr {
    fn eval(&self, env: &Environment) -> LangValue {
        use LangValue::*;
        match self.oper {
            TokenType::Star | TokenType::Slash | TokenType::Plus | TokenType::Minus => {
                let left_eval = self.left.eval(&env);
                let left = match left_eval {
                    LangNumber(x) => x,
                    _ => panic!("NaN {}", left_eval),
                };
                let right_eval = self.right.eval(&env);
                let right = match right_eval {
                    LangNumber(x) => x,
                    _ => panic!("NaN {}", right_eval),
                };

                LangNumber(match self.oper {
                    TokenType::Plus => left + right,
                    TokenType::Minus => left - right,
                    TokenType::Star => left * right,
                    TokenType::Slash => left / right,
                    _ => panic!("Cannot handle {:?}", self.oper),
                })
            },
            _ => panic!("Cannot eval {:?}", self.oper),
        }
    }
}

impl Evaluatable for Identifier {
  fn eval(&self, env: &Environment) -> LangValue {
    *env.get(&self.name).unwrap()
  }
}
impl Expr for Identifier {}
