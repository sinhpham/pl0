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

  
    let tokens = r_lexer("VAR x, squ;

PROCEDURE square;
BEGIN
   squ:= x * x
END;

BEGIN
   x := 1;
   WHILE x <= 10 DO
   BEGIN
      CALL square;
      ! squ;
      x := x + 1
   END
END.");


    let ast = parse_only(program, &tokens);
    
    if let Ok(c) = ast {
        let interpreter = Interpreter::new(c);
        let ret = interpreter.run();
        println!("ret = {:?}", ret);
    }
    
}