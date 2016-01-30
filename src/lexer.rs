use regex::Regex;

use chomp::*;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
pub enum Token<'a> {
    Number(i32),
    Ident(&'a str),
    Keyword(&'a str),
    Separator(&'a str),
}

pub fn r_lexer(input: &str) -> Result<Vec<Token>, String> {
    fn r_number(input: &str) -> Option<(Token, usize, usize)> {
        let re = Regex::new(r"^\d+").unwrap();
        
        if let Some((start, end)) = re.find(input) {
            let num = input[start..end].parse::<i32>().unwrap();
            return Some((Token::Number(num), start, end));
        }
        None
    }
    
    fn r_ident_keyword(input: &str) -> Option<(Token, usize, usize)> {
        let keywords = {
            let mut kw = HashSet::new();
            kw.insert("BEGIN");
            kw.insert("END");
            kw.insert("PROCEDURE");
            kw.insert("WHILE");
            kw.insert("DO");
            kw.insert("IF");
            kw.insert("THEN");
            kw.insert("CALL");
            kw.insert("ODD");
            kw.insert("VAR");
            kw.insert("CONST");
            
            kw
        }; 
        
        let re = Regex::new(r"^[:alpha:]([:alpha:]|\d)*").unwrap();
        
        if let Some((start, end)) = re.find(input) {
            let value = &input[start..end];
            
            if keywords.contains(value) {
                return Some((Token::Keyword(value), start, end))
            }
            
            return Some((Token::Ident(value), start, end));
        }
        None
    }
    
    fn r_sep(input: &str) -> Option<(Token, usize, usize)> {
        let re = Regex::new(r"^(:=)|(>=)|(<=)|(,)|(.)|(;)|(=)|(>)|(<)|(\+)|(-)|(\*)|(/)|(#)|(!)|(\()|(\))").unwrap();
        
        if let Some((start, end)) = re.find(input) {
            return Some((Token::Separator(&input[start..end]), start, end));
        }
        None
    }
    
    fn r_whitespace(input: &str) -> Option<(usize, usize)> {
        let re = Regex::new(r"^\s+").unwrap();
        
        re.find(input)
    }
    
    let mut ret = vec![];
    
    let mut curr_idx: usize = 0;
    let mut curr_str = &input[curr_idx..];
    
    let f1 = &r_ident_keyword;
    let f2 = &r_number;
    let f3 = &r_sep;
    let m_funcs = {
        let mut m_funcs: Vec<&Fn(&str) -> Option<(Token, usize, usize)>> = Vec::new();
        m_funcs.push(f1);
        m_funcs.push(f2);
        m_funcs.push(f3);
        
        m_funcs
    };
    
    while !curr_str.is_empty() {
        let mut progressed = false;
        
        if let Some((_, non_empty)) = r_whitespace(curr_str) {
            curr_idx += non_empty;
            curr_str = &input[curr_idx..];
        }
        
        for m_func in &m_funcs {
            if let Some((token, _, n_start)) = m_func(curr_str) {
                curr_idx += n_start;
                curr_str = &input[curr_idx..];
                ret.push(token);
                progressed = true;
                break;
            }
        }
        
        if !progressed {
            return Err("can not progress".to_string());
        }
    }
    
    Ok(ret)
}

#[test]
fn test_r_lexer() {
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

    assert!(tokens.is_ok());
}

#[test]
fn test_r_lexer2() {
    let tokens = r_lexer("
VAR x, squ;

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

    assert!(tokens.is_ok());
}