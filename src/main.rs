#[macro_use]
extern crate chomp;
extern crate regex;

use chomp::*;

mod lexer;
mod parser;
mod codegen;
mod interpreter;

use regex::Regex;
use lexer::*;
use parser::*;
use interpreter::*;

fn main() {
   
//     let p = parse_only(run_lexer, "
// CONST
//   m =  7,
//   n = 85;".as_bytes());
  
    let r = r_lexer("1234556 d");
    println!("{:?}", r);
    //println!("{:?}", p);
    // {
    //     if let Ok(tokens) = p {
    //         //let input = Input::new(&tokens);
    //         // println!("{:?}", tokens);
    //         let ast = parse_only(program, &tokens);
    //         // println!("{:?}", ast);
    //         
    //         if let Ok(c) = ast {
    //             let interpreter = Interpreter::new(c);
    //             let ret = interpreter.run();
    //             println!("ret = {:?}", ret);
    //         }
    //     }
    // }
    
}