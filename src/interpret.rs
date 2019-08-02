use alloc::{string::String, collections::btree_map::BTreeMap};
use crate::value::LangValue;

pub type Environment = BTreeMap<String, LangValue>;

// trait Executable {
//     fn exec(&self, env: &mut Environment);
// }
// trait Evaluatable {
//     fn eval(&self, env: &Environment) -> LangValue;
// }

// impl Evaluatable for Scope {
//     fn eval(&self, env: &Environment) -> LangValue {
//     let tmp_env = BTreeMap::new();
//         for i in 0..self.lines.len() {
//           let ret = match line {
//             DeclOrExpr::Declaration(decl) => {
//               decl.exec(tmp_env)
//             }
//           };
//           if i == self.lines.len()-1 {
//             return len;
//           }
//         };
//     }
// }
