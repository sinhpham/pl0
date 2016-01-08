#[macro_use]
extern crate chomp;

use chomp::*;

mod lexer;
mod parser;

use lexer::*;
use parser::*;

fn main() {
    // let n = parse_only(number, "  222  ".as_bytes());
    // let i = parse_only(ident, " a  ".as_bytes());
    // let f = parse_only(factor, "  asdf222 * 23".as_bytes());
    // let t = parse_only(term, " aa  *  bb ".as_bytes());
    // let e = parse_only(expression, " (a-2) * b + c-5 +e-f*    xxx /xxx *yyy".as_bytes());
    //let c = parse_only(condition, " x <= 10".as_bytes());
    //let s = parse_only(statement, " x := 1".as_bytes());
    
     //let wd = parse_only(while_do, "WHILE x <= 10 DO x := 1".as_bytes());
  
    
   
    let p = parse_only(run_lexer, "x = asf".as_bytes());
    println!("{:?}", p);
}