use chomp::*;
use std::str;

#[derive(Debug, Clone)]
pub enum Token {
    Number(i32),
    Ident(String),
    Keyword(String),
    Separator(String),
}

fn number(i: Input<u8>) -> U8Result<Token> {
    parse!{i;
        let num = take_while1(|c| (c as char).is_digit(10));

        ret Token::Number(str::from_utf8(num).unwrap().parse::<i32>().unwrap())
    }
}

fn ident(i: Input<u8>) -> U8Result<Token> {
    parse!{i;
        let first = satisfy(|c| (c as char).is_alphabetic() || c == b'_');
        let rest = take_while(|c| (c as char).is_alphabetic() || (c as char).is_digit(10));

        ret Token::Ident(str::from_utf8(&vec![first]).unwrap().to_owned() + str::from_utf8(rest).unwrap())
    }
}

macro_rules! alt {
    ($i:expr, $a:expr) => { $a };
    ($i:expr, $a:expr, $b:expr) => { or($i, $a, $b) };
    ($i:expr, $a:expr, $($b:expr),*) => { or($i, $a, |i| alt!(i, $($b),*)) };
}

fn keyword(i: Input<u8>) -> U8Result<Token> {
    fn to_kw(s: &[u8]) -> Token {
        Token::Keyword(str::from_utf8(s).unwrap().to_owned())
    }
    
    fn all_kw(i: Input<u8>) -> U8Result<Token> {
        alt!(i,
            |idx| string(idx, b"BEGIN").map(to_kw),
            |idx| string(idx, b"END").map(to_kw),
            |idx| string(idx, b"PROCEDURE").map(to_kw),
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
        Token::Separator(str::from_utf8(s).unwrap().to_owned())
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
            |idx| string(idx, b".").map(to_sep)
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
    
    parse!{i;
        let tokens = sep_by1(is_token, is_not_token);

        ret tokens
    }
}

#[test]
fn test_number() {
    let x = parse_only(number, b"123");
    println!("{:?}", x);
    assert!(x.is_ok());
}