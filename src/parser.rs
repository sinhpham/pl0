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
    Equal,
    NumberSign,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

#[derive(Debug, Clone)]
pub enum AstNode {
    Number(i32),
    Ident(String),
    Factor(Box<AstNode>),
    Term{factors: Vec<AstNode>, ops: Vec<BiOp>},
    Expression{terms: Vec<AstNode>, signs: Vec<Sign>},
    ComposedExpression{ex1: Box<AstNode>, op: ExOp, ex2: Box<AstNode>},
    BeginEnd(Vec<AstNode>),
    IfThen{condition: Box<AstNode>, statement: Box<AstNode>},
    WhileDo{condition: Box<AstNode>, statement: Box<AstNode>},
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

macro_rules! alt {
    ($i:expr, $a:expr) => { $a };
    ($i:expr, $a:expr, $b:expr) => { or($i, $a, $b) };
    ($i:expr, $a:expr, $($b:expr),*) => { or($i, $a, |i| alt!(i, $($b),*)) };
}

fn ex_op(i: Input<u8>) -> U8Result<ExOp> {
    alt!(i,
        less_than_or_equal,
        greater_than_or_equal,
        equal,
        number_sign,
        less_than,
        greater_than)
}

fn equal(i: Input<u8>) -> U8Result<ExOp> {
    parse!{i;
        let _ = token(b'=');

        ret ExOp::Equal
    }
}

fn number_sign(i: Input<u8>) -> U8Result<ExOp> {
    parse!{i;
        let _ = token(b'#');

        ret ExOp::NumberSign
    }
}

fn less_than(i: Input<u8>) -> U8Result<ExOp> {
    parse!{i;
        let _ = token(b'<');

        ret ExOp::LessThan
    }
}

fn less_than_or_equal(i: Input<u8>) -> U8Result<ExOp> {
    parse!{i;
        let _ = string(b"<=");

        ret ExOp::LessThanOrEqual
    }
}

fn greater_than(i: Input<u8>) -> U8Result<ExOp> {
    parse!{i;
        let _ = token(b'>');

        ret ExOp::GreaterThan
    }
}

fn greater_than_or_equal(i: Input<u8>) -> U8Result<ExOp> {
    parse!{i;
        let _ = string(b">=");

        ret ExOp::GreaterThanOrEqual
    }
}

fn number(i: Input<u8>) -> U8Result<AstNode> {
    parse!{i;
        let _ = take_while(|c| (c as char).is_whitespace());
        
        let num = take_while1(|c| (c as char).is_digit(10));

        ret AstNode::Number(str::from_utf8(num).unwrap().parse::<i32>().unwrap())
    }
}

fn ident(i: Input<u8>) -> U8Result<AstNode> {
    parse!{i;
        let _ = take_while(|c| (c as char).is_whitespace());
        
        let first = satisfy(|c| (c as char).is_alphabetic());
        let rest = take_while(|c| (c as char).is_alphabetic() || (c as char).is_digit(10));

        ret AstNode::Ident(str::from_utf8(&vec![first]).unwrap().to_owned() + str::from_utf8(rest).unwrap())
    }
}

fn factor(i: Input<u8>) -> U8Result<AstNode> {
    fn grouped_expression(i: Input<u8>) -> U8Result<AstNode> {
        parse!{i;
            let _ = take_while(|c| (c as char).is_whitespace());
            
            token(b'(');
            let e = expression();
            token(b')');
            
            ret e
        }
    }
    fn numer_or_ident(i: Input<u8>) -> U8Result<AstNode> {
        parse!{i;
            let _ = take_while(|c| (c as char).is_whitespace());
            
            let r = or(number, ident);
            
            ret r
        }
    }
    parse!{i;
        let _ = take_while(|c| (c as char).is_whitespace());
    
        let f = or(numer_or_ident, grouped_expression);
        
        ret AstNode::Factor(Box::new(f))
    }
}

fn term(i: Input<u8>) -> U8Result<AstNode> {
    fn sub_term(i: Input<u8>) -> U8Result<(BiOp, AstNode)> {
        parse!{i;
            let _ = take_while(|c| (c as char).is_whitespace());
            
            let sign = or(mul_sign, div_sign);
            let fa = factor();
            
            ret (sign, fa)
        }
    }
    
    parse!{i;
        let _ = take_while(|c| (c as char).is_whitespace());
        
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
            let _ = take_while(|c| (c as char).is_whitespace());
            
            let sign = sign();
            let term = term();
            ret (term, sign)
        }
    }
    
    parse!{i;
        let _ = take_while(|c| (c as char).is_whitespace());
        
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
            let _ = take_while(|c| (c as char).is_whitespace());
            
            let _ = string(b"ODD");
            let ex = expression();
            ret ex
        }
    }

    fn composed_expression(i: Input<u8>) -> U8Result<AstNode> {
        parse!{i;
            let _ = take_while(|c| (c as char).is_whitespace());

            let ex1 = expression();
            let _ = take_while(|c| (c as char).is_whitespace());
            let op = ex_op();
            let ex2 = expression();
            ret AstNode::ComposedExpression{ex1: Box::new(ex1), op: op, ex2: Box::new(ex2)}
        }
    }
    
    parse!{i;
        let _ = take_while(|c| (c as char).is_whitespace());
        
        let ret = or(odd_expression, composed_expression);
        ret ret
    }
}



fn statement(i: Input<u8>) -> U8Result<AstNode> {
    fn assignment(i: Input<u8>) -> U8Result<AstNode> {
        parse!{i;
            let _ = take_while(|c| (c as char).is_whitespace());
            
            let _ = ident();
            let _ = take_while(|c| (c as char).is_whitespace());
            string(b":=");
            let _ = take_while(|c| (c as char).is_whitespace());
            let ex = expression();
            ret ex
        }
    }
    
    fn call(i: Input<u8>) -> U8Result<AstNode> {
        parse!{i;
            let _ = take_while(|c| (c as char).is_whitespace());
            
            string(b"CALL");
            let _ = take_while(|c| (c as char).is_whitespace());
            let ident = ident();
            ret ident
        }
    }
    
    fn question_mark(i: Input<u8>) -> U8Result<AstNode> {
        parse!{i;
            let _ = take_while(|c| (c as char).is_whitespace());
            
            token(b'?');
            let ident = ident();
            ret ident
        }
    }
    
    fn exclaimation(i: Input<u8>) -> U8Result<AstNode> {
        parse!{i;
            let _ = take_while(|c| (c as char).is_whitespace());
            
            token(b'!');
            let _ = take_while(|c| (c as char).is_whitespace());
            let ex = expression();
            ret ex
        }
    }
    
    fn begin_end_block(i: Input<u8>) -> U8Result<AstNode> {
        parse!{i;
            let _ = take_while(|c| (c as char).is_whitespace());
            
            string(b"BEGIN");
            let _ = take_while(|c| (c as char).is_whitespace());
            let statements: Vec<AstNode> = sep_by1(statement, |idx| token(idx, b';'));
            let _ = take_while(|c| (c as char).is_whitespace());
            string(b"END");
            
            ret AstNode::BeginEnd({
                statements
            })
        }
    }
    
    fn if_then(i: Input<u8>) -> U8Result<AstNode> {
        parse!{i;
            let _ = take_while(|c| (c as char).is_whitespace());
            
            string(b"IF");
            let cod = condition();
            let _ = take_while(|c| (c as char).is_whitespace());
            string(b"THEN");
            let st = statement();
            
            ret AstNode::IfThen {
                condition: Box::new(cod),
                statement: Box::new(st)
            }
        }
    }
    
    fn while_do(i: Input<u8>) -> U8Result<AstNode> {
        parse!{i;
            let _ = take_while(|c| (c as char).is_whitespace());
            
            string(b"WHILE");
            let cod = condition();
            let _ = take_while(|c| (c as char).is_whitespace());
            string(b"DO");
            let st = statement();
            
            ret AstNode::WhileDo {
                condition: Box::new(cod),
                statement: Box::new(st)
            }
        }
    }
    
    fn all_choices(i: Input<u8>) -> U8Result<AstNode> {
        alt!(i,
        assignment,
        call,
        question_mark,
        exclaimation,
        begin_end_block,
        if_then,
        while_do)
    }
        
    parse!{i;
        let _ = take_while(|c| (c as char).is_whitespace());
        
        let s = option(all_choices, AstNode::Number(0));
        ret s
    }
}

fn block(i: Input<u8>) -> U8Result<AstNode> {
    fn const_declaration(i: Input<u8>) -> U8Result<AstNode> {
        fn sub_const_decl(i: Input<u8>) -> U8Result<AstNode> {
            parse!{i;
                let _ = take_while(|c| (c as char).is_whitespace());
                
                let id = ident();
                let _ = take_while(|c| (c as char).is_whitespace());
                token(b'=');
                let _ = take_while(|c| (c as char).is_whitespace());
                let _ = number();
                
                ret id
            }
        }
        
        parse!{i;
            let _ = take_while(|c| (c as char).is_whitespace());
            
            string(b"CONST");
            let subs: Vec<AstNode> = sep_by1(sub_const_decl, |idx| token(idx, b','));
            token(b';');
            
            ret subs[0].clone()
        }
    }
    
    fn var_declaration(i: Input<u8>) -> U8Result<AstNode> {
        parse!{i;
            let _ = take_while(|c| (c as char).is_whitespace());
            
            string(b"VAR");
            let subs: Vec<AstNode> = sep_by1(ident, |idx| token(idx, b','));
            token(b';');
            
            ret subs[0].clone()
        }
    }
    
    fn procedure(i: Input<u8>) -> U8Result<AstNode> {
        parse!{i;
            let _ = take_while(|c| (c as char).is_whitespace());
            
            string(b"PROCEDURE");
            let _ = ident();
            token(b';');
            let block = block();
            token(b';');
            
            ret block
        }
    }
    
    parse!{i;
        let _ = take_while(|c| (c as char).is_whitespace());
        
        let _ = option(const_declaration, AstNode::Number(0));
        let _ = option(var_declaration, AstNode::Number(0));
        let p: Vec<AstNode> = many(procedure);
        let s = statement();
        ret s
    }
}

pub fn program(i: Input<u8>) -> U8Result<AstNode> {
    parse!{i;
        let _ = take_while(|c| (c as char).is_whitespace());
        
        let block = block();
        token(b'.');
        ret block
    }
}