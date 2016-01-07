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
  
    
   
    let p = parse_only(run_lexer, "CONST
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
  x := 25;
  y :=  3;
  CALL divide;
  x := 84;
  y := 36;
  CALL gcd
END . sd ".as_bytes());
    println!("{:?}", p);
}