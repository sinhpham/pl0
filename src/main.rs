#[macro_use]
extern crate chomp;

use chomp::*;
use std::str;

#[derive(Debug)]
enum Sign {
    Minus
}

#[derive(Debug)]
enum BiOp {
    Add,
    Sub,
    Mul,
    Div
}

#[derive(Debug)]
enum AstNode {
    Number(i32),
    Ident(String),
    Factor(Box<AstNode>),
    Term{factors: Vec<AstNode>, ops: Vec<BiOp>},
    Expression{terms: Vec<AstNode>, signs: Vec<Sign>}
}

fn number(i: Input<u8>) -> U8Result<AstNode> {
    parse!{i;
        let num = take_while1(|c| (c as char).is_digit(10));

        ret AstNode::Number(str::from_utf8(num).unwrap().parse::<i32>().unwrap())
    }
}

fn ident(i: Input<u8>) -> U8Result<AstNode> {
    parse!{i;
        let first = satisfy(|c| (c as char).is_alphabetic());
        let rest = take_while1(|c| (c as char).is_alphabetic() || (c as char).is_digit(10));

        ret AstNode::Ident(str::from_utf8(&vec![first]).unwrap().to_owned() + str::from_utf8(rest).unwrap())
    }
}

fn bi_op(i: Input<u8>) -> U8Result<BiOp> {
    parse!{i;
        let first = satisfy(|c| {
            let ch = c as char;
            ch == '*' || ch == '/' || ch == '+' || ch == '-'
        });

        ret BiOp::Mul
    }
}

fn factor(i: Input<u8>) -> U8Result<AstNode> {
    parse!{i;
        let f = or(number, ident);
        
        ret AstNode::Factor(Box::new(f))
    }
}

fn term(i: Input<u8>) -> U8Result<AstNode> {
    parse!{i;
        let f: Vec<AstNode> = sep_by(factor, |iner_i| token(iner_i, b'*'));
        
        ret AstNode::Term {
            factors: f,
            ops: vec![BiOp::Add]
        }
    }
}

fn sub_expression(i: Input<u8>) -> U8Result<AstNode> {
    parse!{i;
        let e_sign = matched_by(
                |iner_i| token(iner_i, b'-'));
        let (_, term_ret) = matched_by(term);
        ret term_ret
    }
}

fn expression(i: Input<u8>) -> U8Result<AstNode> {
    parse!{i;
        let e: Vec<AstNode> = many1(sub_expression);
    
        ret AstNode::Expression {
            terms: e,
            signs: vec![Sign::Minus]
        }
    }
}


fn main() {
    let n = parse_only(number, "222  ".as_bytes());
    let x = parse_only(ident, "asdf222  ".as_bytes());
    let f = parse_only(factor, "asdf222 23".as_bytes());
    let t = parse_only(term, "22233*asdf2*23".as_bytes());
    let e = parse_only(expression, "-22233*asdf2*23-x-y".as_bytes());
    println!("{:?}", e);
}
