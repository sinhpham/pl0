use regex::Regex;

use chomp::*;
//use chomp::parsers::Error;
use std::str;
use std::cell::Cell;
use std::collections::HashSet;
use std::io;

#[derive(Debug, Clone, Copy)]
pub enum Token<'a> {
    Number(i32),
    Ident(&'a str),
    Keyword(&'a str),
    Separator(&'a str),
}

fn number(i: Input<u8>) -> U8Result<Token> {
    parse!{i;
        let num = take_while1(|c| (c as char).is_digit(10));

        ret Token::Number(str::from_utf8(num).unwrap().parse::<i32>().unwrap())
    }
}

fn ident(i: Input<u8>) -> U8Result<Token> {
    let first_run = Cell::new(true);
    
    parse!{i;
        let rest = take_while(|c| {
            if first_run.get() {
                first_run.set(false);
                (c as char).is_alphabetic() || c == b'_'
            } else {
                (c as char).is_alphabetic() || (c as char).is_digit(10)
            }
        });

        ret Token::Ident(str::from_utf8(rest).unwrap())
    }
}

fn keyword(i: Input<u8>) -> U8Result<Token> {
    fn to_kw(s: &[u8]) -> Token {
        Token::Keyword(str::from_utf8(s).unwrap())
    }
    
    fn all_kw(i: Input<u8>) -> U8Result<Token> {
        
        parse!{i;
            let r = string(b"BEGIN")
                <|> string(b"END")
                <|> string(b"PROCEDURE")
                <|> string(b"WHILE")
                <|> string(b"DO")
                <|> string(b"IF")
                <|> string(b"THEN")
                <|> string(b"CALL")
                <|> string(b"ODD")
                <|> string(b"VAR")
                <|> string(b"CONST");
                
            ret to_kw(r)
        }
    }
    
    parse!{i;
        let kw = all_kw();

        ret kw
    }
}

fn separator(i: Input<u8>) -> U8Result<Token> {
    fn to_sep(s: &[u8]) -> Token {
        Token::Separator(str::from_utf8(s).unwrap())
    }
    
    fn all_sep(i: Input<u8>) -> U8Result<Token> {
        parse!{i;
            let r = string(b":=")
                <|> string(b">=")
                <|> string(b"<=")
                <|> string(b",")
                <|> string(b";")
                <|> string(b"=")
                <|> string(b">")
                <|> string(b"<")
                <|> string(b"+")
                <|> string(b"-")
                <|> string(b"*")
                <|> string(b"/")
                <|> string(b"#")
                <|> string(b".")
                <|> string(b"!")
                <|> string(b"(")
                <|> string(b")");
            
            ret to_sep(r)
        }
    }
    
    parse!{i;
        let sep = all_sep();

        ret sep
    }
}

pub fn run_lexer(i: Input<u8>) -> U8Result<Vec<Token>> {
    fn is_token(i: Input<u8>) -> U8Result<Token> {
        // println!("tok called");
        // println!("i = {:?}", i);
        // let mut input_text = String::new();
        // io::stdin()
        //             .read_line(&mut input_text);
        parse!{i;
            let r = keyword() <|> separator() <|> number() <|> ident();

            ret r
        }
    }
    
    fn is_not_token(i: Input<u8>) -> U8Result<()> {
        // println!("not tok called");
        // println!("i = {:?}", i);
        // let mut input_text = String::new();
        // io::stdin()
        //             .read_line(&mut input_text);
        parse!{i;
            let _ = take_while(|c| (c as char).is_whitespace());

            ret ()
        }
    }
    
    fn all_token(i: Input<u8>) -> U8Result<Token> {
        parse!{i;
            let _ = option(is_not_token, ());
            let tok = is_token();

            ret tok
        }
    }
    
    parse!{i;
        //let _ = take_while(|c| (c as char).is_whitespace());
        let tokens = many(all_token);

        ret tokens
    }
}



pub fn r_lexer(input: &str) -> Vec<Token> {
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
    
    // let m_funcs = vec![&r_keyword, &r_sep, &r_number, Box::new(r_ident)];
    
    let f1 = &r_ident_keyword;
    let f2 = &r_number;
    let f3 = &r_sep;
    
    let d = f1(input);
    // println!("d = {:?}", d);
    
    let mut m_funcs: Vec<&Fn(&str) -> Option<(Token, usize, usize)>> = Vec::new();
    
    m_funcs.push(f1);
    m_funcs.push(f2);
    m_funcs.push(f3);
    
    while !curr_str.is_empty() {
        // println!("while curr_str = {:?}", curr_str);
        
        if let Some((_, non_empty)) = r_whitespace(curr_str) {
            curr_idx += non_empty;
            curr_str = &input[curr_idx..];
            // println!("ws curr_str = {:?}", curr_str);
        }
        
        for m_func in &m_funcs {
            // println!("call f");
            if let Some((t, _, n_start)) = m_func(curr_str) {
                curr_idx += n_start;
                curr_str = &input[curr_idx..];
                // println!("f curr_str = {:?}", curr_str);
                r.push(t);
                break;
            }
        }
//         let mut input = String::new();
// 
//         (io::stdin().read_line(&mut input));
        //panic!("shouldn't be here");
    }
    
    r
}

#[test]
fn test_number() {
    let x = parse_only(number, b"123");
    println!("{:?}", x);
    assert!(x.is_ok());
}