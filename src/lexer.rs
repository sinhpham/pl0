use regex::Regex;

use chomp::*;
use std::str;
use std::cell::Cell;
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
    
    
    let mut r = vec![];
    
    let mut curr_idx:usize = 0;
    let mut curr_str = &input[curr_idx..];
    
    let f1 = &r_ident_keyword;
    let f2 = &r_number;
    let f3 = &r_sep;
    
    let d = f1(input);
    
    let mut m_funcs: Vec<&Fn(&str) -> Option<(Token, usize, usize)>> = Vec::new();
    
    m_funcs.push(f1);
    m_funcs.push(f2);
    m_funcs.push(f3);
    
    while !curr_str.is_empty() {
        let mut progressed = false;
        
        if let Some((_, non_empty)) = r_whitespace(curr_str) {
            curr_idx += non_empty;
            curr_str = &input[curr_idx..];
        }
        
        for m_func in &m_funcs {
            if let Some((t, _, n_start)) = m_func(curr_str) {
                curr_idx += n_start;
                curr_str = &input[curr_idx..];
                r.push(t);
                progressed = true;
                break;
            }
        }
        
        if !progressed {
            return Err("can not progress".to_string());
        }
    }
    
    Ok(r)
}

#[test]
fn test_number() {
    let x = parse_only(number, b"123");
    println!("{:?}", x);
    assert!(x.is_ok());
}