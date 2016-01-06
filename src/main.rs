#[macro_use]
extern crate chomp;

use chomp::*;
use std::str;

#[derive(Debug, Clone)]
enum Sign {
    Plus,
    Minus
}

#[derive(Debug, Clone)]
enum BiOp {
    Mul,
    Div
}

#[derive(Debug, Clone)]
enum ExOp {
    Op,
}

#[derive(Debug, Clone)]
enum AstNode {
    Number(i32),
    Ident(String),
    Factor(Box<AstNode>),
    Term{factors: Vec<AstNode>, ops: Vec<BiOp>},
    Expression{terms: Vec<AstNode>, signs: Vec<Sign>},
    ComposedExpression{ex1: Box<AstNode>, op: ExOp, ex2: Box<AstNode>},
}

fn plus_sign(i: Input<u8>) -> U8Result<Sign> {
    parse!{i;
        let _ = token(b'+');

        ret Sign::Plus
    }
}

fn minus_sign(i: Input<u8>) -> U8Result<Sign> {
    parse!{i;
        let _ = token(b'-');

        ret Sign::Minus
    }
}

fn sign(i: Input<u8>) -> U8Result<Sign> {
    parse!{i;
        let e_sign = or(plus_sign,minus_sign);
        
        ret e_sign
    }
}

fn mul_sign(i: Input<u8>) -> U8Result<BiOp> {
    parse!{i;
        let _ = token(b'*');

        ret BiOp::Mul
    }
}

fn div_sign(i: Input<u8>) -> U8Result<BiOp> {
    parse!{i;
        let _ = token(b'/');

        ret BiOp::Div
    }
}

fn ex_op(i: Input<u8>) -> U8Result<ExOp> {
    parse!{i;
        let o = take_while1(|c| c == b'=');

        ret ExOp::Op
    }
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
        let rest = take_while(|c| (c as char).is_alphabetic() || (c as char).is_digit(10));

        ret AstNode::Ident(str::from_utf8(&vec![first]).unwrap().to_owned() + str::from_utf8(rest).unwrap())
    }
}

fn factor(i: Input<u8>) -> U8Result<AstNode> {
    fn grouped_expression(i: Input<u8>) -> U8Result<AstNode> {
        parse!{i;
            token(b'(');
            let e = expression();
            token(b')');
            
            ret e
        }
    }
    fn numer_or_ident(i: Input<u8>) -> U8Result<AstNode> {
        parse!{i;
            let r = or(number, ident);
            
            ret r
        }
    }
    parse!{i;
        let f = or(numer_or_ident, grouped_expression);
        
        ret AstNode::Factor(Box::new(f))
    }
}

fn term(i: Input<u8>) -> U8Result<AstNode> {
    fn sub_term(i: Input<u8>) -> U8Result<(BiOp, AstNode)> {
        parse!{i;
            let sign = or(mul_sign, div_sign);
            let fa = factor();
            
            ret (sign, fa)
        }
    }
    
    parse!{i;
        let first_factor = factor();
        
        let sub_terms: Vec<(BiOp, AstNode)> = many(sub_term);
        
        ret AstNode::Term {
            factors: {
                let mut v = vec![first_factor];
                for t in &sub_terms {
                    let (_, x) = t.clone();
                    v.push(x.clone());
                }
                v
            },
            ops: {
                let mut v = vec![];
                for t in &sub_terms {
                    let (x, _) = t.clone();
                    v.push(x.clone());
                }
                v
            }
        }
    }
}

fn expression(i: Input<u8>) -> U8Result<AstNode> {
    fn sub_expression(i: Input<u8>) -> U8Result<(AstNode, Sign)> {
        parse!{i;
            let sign = sign();
            let term = term();
            ret (term, sign)
        }
    }
    
    parse!{i;
        let first_sign = option(sign, Sign::Plus);
        let first_term = term();
    
        let e: Vec<(AstNode, Sign)> = many(sub_expression);
    
        ret AstNode::Expression {
            terms: {
                let mut v = vec![first_term];
                for t in &e {
                    let (x, _) = t.clone();
                    v.push(x.clone());
                }
                v
            },
            signs: {
                let mut v = vec![first_sign];
                for t in &e {
                    let (_, x) = t.clone();
                    v.push(x.clone());
                }
                v
            }
        }
    }
}

fn condition(i: Input<u8>) -> U8Result<AstNode> {
    fn odd_expression(i: Input<u8>) -> U8Result<AstNode> {
        parse!{i;
            let _ = string(b"odd");
            let ex = expression();
            ret ex
        }
    }

    fn composed_expression(i: Input<u8>) -> U8Result<AstNode> {
        parse!{i;

            let ex1 = expression();
            let op = ex_op();
            let ex2 = expression();
            ret AstNode::ComposedExpression{ex1: Box::new(ex1), op: op, ex2: Box::new(ex2)}
        }
    }
    
    parse!{i;
        let ret = or(odd_expression, composed_expression);
        ret ret
    }
}

fn statement(i: Input<u8>) -> U8Result<AstNode> {
    fn assignment(i: Input<u8>) -> U8Result<AstNode> {
        unimplemented!();
    }
    
    fn call(i: Input<u8>) -> U8Result<AstNode> {
        unimplemented!();
    }
    
    fn question_mark(i: Input<u8>) -> U8Result<AstNode> {
        unimplemented!();
    }
    
    fn exclaimation(i: Input<u8>) -> U8Result<AstNode> {
        unimplemented!();
    }
    
    fn begin_end_block(i: Input<u8>) -> U8Result<AstNode> {
        unimplemented!();
    }
    
    fn if_then(i: Input<u8>) -> U8Result<AstNode> {
        unimplemented!();
    }
    
    fn while_do(i: Input<u8>) -> U8Result<AstNode> {
        unimplemented!();
    }
    unimplemented!();
}

fn block(i: Input<u8>) -> U8Result<AstNode> {
    fn const_declaration(i: Input<u8>) -> U8Result<AstNode> {
        unimplemented!();
    }
    
    fn var(i: Input<u8>) -> U8Result<AstNode> {
        unimplemented!();
    }
    
    fn procedure(i: Input<u8>) -> U8Result<AstNode> {
        unimplemented!();
    }
    unimplemented!();
}

fn program(i: Input<u8>) -> U8Result<AstNode> {
    parse!{i;
        let block = block();
        token(b'.');
        ret block
    }
}


fn main() {
    let n = parse_only(number, "222  ".as_bytes());
    let x = parse_only(ident, "a".as_bytes());
    let f = parse_only(factor, "asdf222 23".as_bytes());
    let t = parse_only(term, "aa*bb".as_bytes());
    let e = parse_only(expression, "(a-2)*b+c-5+e-f*xxx/xxx*yyy".as_bytes());
    let c = parse_only(condition, "a=x*y+5".as_bytes());
    println!("{:?}", c);
}
