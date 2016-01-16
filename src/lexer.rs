use chomp::*;
//use chomp::parsers::Error;
use std::str;
use std::cell::Cell;

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

macro_rules! alt {
    ($i:expr, $a:expr) => { $a };
    ($i:expr, $a:expr, $b:expr) => { or($i, $a, $b) };
    ($i:expr, $a:expr, $($b:expr),*) => { or($i, $a, |i| alt!(i, $($b),*)) };
}

fn keyword(i: Input<u8>) -> U8Result<Token> {
    fn to_kw(s: &[u8]) -> Token {
        Token::Keyword(str::from_utf8(s).unwrap())
    }
    
    fn all_kw(i: Input<u8>) -> U8Result<Token> {
        alt!(i,
            |idx| string(idx, b"BEGIN").map(to_kw),
            |idx| string(idx, b"END").map(to_kw),
            |idx| string(idx, b"PROCEDURE").map(to_kw),
            |idx| string(idx, b"WHILE").map(to_kw),
            |idx| string(idx, b"DO").map(to_kw),
            |idx| string(idx, b"IF").map(to_kw),
            |idx| string(idx, b"THEN").map(to_kw),
            |idx| string(idx, b"CALL").map(to_kw),
            |idx| string(idx, b"ODD").map(to_kw),
            |idx| string(idx, b"VAR").map(to_kw),
            |idx| string(idx, b"CONST").map(to_kw)
        )
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
        alt!(i,
            |idx| string(idx, b":=").map(to_sep),
            |idx| string(idx, b">=").map(to_sep),
            |idx| string(idx, b"<=").map(to_sep),
            |idx| string(idx, b",").map(to_sep),
            |idx| string(idx, b";").map(to_sep),
            |idx| string(idx, b"=").map(to_sep),
            |idx| string(idx, b">").map(to_sep),
            |idx| string(idx, b"<").map(to_sep),
            |idx| string(idx, b"+").map(to_sep),
            |idx| string(idx, b"-").map(to_sep),
            |idx| string(idx, b"*").map(to_sep),
            |idx| string(idx, b"/").map(to_sep),
            |idx| string(idx, b"#").map(to_sep),
            |idx| string(idx, b".").map(to_sep),
            |idx| string(idx, b"!").map(to_sep),
            |idx| string(idx, b"(").map(to_sep),
            |idx| string(idx, b")").map(to_sep)
        )
    }
    
    parse!{i;
        let sep = all_sep();

        ret sep
    }
}

pub fn run_lexer(i: Input<u8>) -> U8Result<Vec<Token>> {
    fn is_token(i: Input<u8>) -> U8Result<Token> {
        alt!(i,
            keyword,
            separator,
            number,
            ident
        )
    }
    
    fn is_not_token(i: Input<u8>) -> U8Result<()> {
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

#[test]
fn test_number() {
    let x = parse_only(number, b"123");
    println!("{:?}", x);
    assert!(x.is_ok());
}