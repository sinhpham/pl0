#[macro_use]
extern crate chomp;
extern crate regex;

use chomp::*;

mod lexer;
mod parser;
mod codegen;
mod interpreter;

use lexer::*;
use parser::*;
use interpreter::*;

fn main() {

  
    let tokens = r_lexer("
    
CONST
  m =  7,
  n = 85;

VAR
  x, y, z, q, r;

PROCEDURE multiply;
VAR a, b;

BEGIN
  a := x;
  b := y;
  z := 0;
  WHILE b > 0 DO BEGIN
    IF ODD b THEN z := z + a;
    a := 2 * a;
    b := b / 2
  END
END;

PROCEDURE divide;
VAR w;
BEGIN
  r := x;
  q := 0;
  w := y;
  WHILE w <= r DO w := 2 * w;
  WHILE w > y DO BEGIN
    q := 2 * q;
    w := w / 2;
    IF w <= r THEN BEGIN
      r := r - w;
      q := q + 1
    END
  END
END;

PROCEDURE gcd;
VAR f, g;
BEGIN
  f := x;
  g := y;
  WHILE f # g DO BEGIN
    IF f < g THEN g := g - f;
    IF g < f THEN f := f - g
  END;
  z := f
END;

BEGIN
  x := m;
  y := n;
  CALL multiply;
  !z;
  
  x := 25;
  y :=  3;
  CALL divide;
  !r;
  !q;
  
  x := 84;
  y := 36;
  CALL gcd;
  
  !z;
END.");

    // println!("tokens = {:?}", tokens);
    
    if let Ok(tokens) = tokens {
        let ast = parse_only(program, &tokens);
        
        if let Ok(c) = ast {
            // println!("c = {:?}", c);
            
            let interpreter = Interpreter::new(c);
            let ret = interpreter.run();
            println!("ret = {:?}", ret);
        }
    }
    
}