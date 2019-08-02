// use alloc::{vec::Vec, string::String, boxed::Box};
use alloc::{boxed::Box};
use crate::ast::*;
use crate::value::LangValue;
use crate::println;

pub fn test_interpreter() {
    println!("{}", Scope{
        lines: vec![
            DeclOrExpr::Expression(Box::new(LangValue::LangNumber(7.0)) as Box<dyn Expr>),
        ],
    });
//   let text = String::from(r##"
// $ print println getch = {
//   [ch, get] = getch nil;
//   listenInput print println getch line;
// }
// "##);
//   let mut tokens: Vec<Token> = Vec::new();
//   ScannerIter::init(&text).scan(&mut tokens);

//   let mut token_iter = crate::parser::TokenIter::from(tokens);
//   match crate::parser::parse_file(&mut token_iter) {
//     Ok(ast) => {
//       for decl in ast {
//         println!("{}", decl.repr());
//       }
//     },
//     Err(e) => {
//       println!("{}", e);
//       let tok = token_iter.prev();
//       println!("Instead, I got {} on line {}", tok.literal, tok.line);
//       let lines: Vec<&str> = text.split("\n").collect();
//       println!("{}", lines[tok.line])
//     }
//   }
}

// $ print println getch = {
//   listenInput print println getch line;
// }

// listenInput print println getch line = {
//   [ch, getch'] = getch nil;
//   processInput ch line (
//     \[line', env'] -> listenInput print println getch' line' env'
//   );
// }

// processInput ch line env callback ===++
//   case ch of
//     'n' -> {
//       println "";
//       [env', val] = (env-eval env line !);
//       println val;
//       print "# ";
//       callback "" env';
//     }
//     _ -> {
//       print ch;
//       callback (line ++ ch) env;
//     };
