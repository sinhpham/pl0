use chomp::*;
use std::str;

use lexer::*;

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
pub enum AstNode<'a> {
    Number(i32),
    Ident(&'a str),
    Factor(Box<AstNode<'a>>),
    Term{factors: Vec<AstNode<'a>>, ops: Vec<BiOp>},
    Expression{terms: Vec<AstNode<'a>>, signs: Vec<Sign>},
    ComposedExpression{ex1: Box<AstNode<'a>>, op: ExOp, ex2: Box<AstNode<'a>>},
    BeginEnd(Vec<AstNode<'a>>),
    IfThen{condition: Box<AstNode<'a>>, statement: Box<AstNode<'a>>},
    WhileDo{condition: Box<AstNode<'a>>, statement: Box<AstNode<'a>>},
}

fn token_separator_cotent<'a>(tok: Token<'a>) -> Option<&'a str> {
    match tok {
        Token::Separator(tc) => {
            Some(tc)
        },
        _ => None
    }
}

fn token_keyword_cotent<'a>(tok: Token<'a>) -> Option<&'a str> {
    match tok {
        Token::Keyword(tc) => {
            Some(tc)
        },
        _ => None
    }
}

fn plus_sign<'a>(i: Input<'a, Token>) -> SimpleResult<'a, Token<'a>, Sign> {
    parse!{i;
        let _ = satisfy_with(token_separator_cotent, |sep| sep == Some("+"));

        ret Sign::Plus
    }
}

fn minus_sign<'a>(i: Input<'a, Token>) -> SimpleResult<'a, Token<'a>, Sign> {
    parse!{i;
        let _ = satisfy_with(token_separator_cotent, |sep| sep == Some("-"));

        ret Sign::Minus
    }
}

fn sign<'a>(i: Input<'a, Token>) -> SimpleResult<'a, Token<'a>, Sign> {
    parse!{i;
        let e_sign = or(plus_sign, minus_sign);
        
        ret e_sign
    }
}

fn mul_sign<'a>(i: Input<'a, Token>) -> SimpleResult<'a, Token<'a>, BiOp> {
    parse!{i;
        let _ = satisfy_with(token_separator_cotent, |sep| sep == Some("*"));

        ret BiOp::Mul
    }
}

fn div_sign<'a>(i: Input<'a, Token>) -> SimpleResult<'a, Token<'a>, BiOp> {
    parse!{i;
        
        let _ = satisfy_with(token_separator_cotent, |sep| sep == Some("/"));

        ret BiOp::Div
    }
}

macro_rules! alt {
    ($i:expr, $a:expr) => { $a };
    ($i:expr, $a:expr, $b:expr) => { or($i, $a, $b) };
    ($i:expr, $a:expr, $($b:expr),*) => { or($i, $a, |i| alt!(i, $($b),*)) };
}

fn ex_op<'a>(i: Input<'a, Token>) -> SimpleResult<'a, Token<'a>, ExOp> {
    alt!(i,
        less_than_or_equal,
        greater_than_or_equal,
        equal,
        number_sign,
        less_than,
        greater_than)
}

fn equal<'a>(i: Input<'a, Token>) -> SimpleResult<'a, Token<'a>, ExOp> {
    parse!{i;
        let _ = satisfy_with(token_separator_cotent, |sep| sep == Some("="));

        ret ExOp::Equal
    }
}

fn number_sign<'a>(i: Input<'a, Token>) -> SimpleResult<'a, Token<'a>, ExOp> {
    parse!{i;
        let _ = satisfy_with(token_separator_cotent, |sep| sep == Some("#"));

        ret ExOp::NumberSign
    }
}

fn less_than<'a>(i: Input<'a, Token>) -> SimpleResult<'a, Token<'a>, ExOp> {
    parse!{i;
        let _ = satisfy_with(token_separator_cotent, |sep| sep == Some("<"));

        ret ExOp::LessThan
    }
}

fn less_than_or_equal<'a>(i: Input<'a, Token>) -> SimpleResult<'a, Token<'a>, ExOp> {
    parse!{i;
        let _ = satisfy_with(token_separator_cotent, |sep| sep == Some("<="));

        ret ExOp::LessThanOrEqual
    }
}

fn greater_than<'a>(i: Input<'a, Token>) -> SimpleResult<'a, Token<'a>, ExOp> {
    parse!{i;
        let _ = satisfy_with(token_separator_cotent, |sep| sep == Some(">"));

        ret ExOp::GreaterThan
    }
}

fn greater_than_or_equal<'a>(i: Input<'a, Token>) -> SimpleResult<'a, Token<'a>, ExOp> {
    parse!{i;
        let _ = satisfy_with(token_separator_cotent, |sep| sep == Some(">="));

        ret ExOp::GreaterThanOrEqual
    }
}

fn number<'a>(i: Input<'a, Token>) -> SimpleResult<'a, Token<'a>, AstNode<'a>> {
    let n = satisfy(i,
        |t| {
            match t {
                Token::Number(_) => true,
                _ => false
            }
        }).map(|lc| {
            match lc {
                Token::Number(c) => AstNode::Number(c),
                _ => panic!("asd")
            }
        });
    n
}

fn ident<'a>(i: Input<'a, Token>) -> SimpleResult<'a, Token<'a>, AstNode<'a>> {
    let ident = satisfy(i, |t| {
        match t {
            Token::Ident(_) => true,
            _ => false 
        }
    }).map(|lc| {
        match lc {
            Token::Ident(id) => AstNode::Ident(id),
            _ => panic!("asd")
        }
    });

    ident
}

fn factor<'a>(i: Input<'a, Token>) -> SimpleResult<'a, Token<'a>, AstNode<'a>> {
    fn grouped_expression<'a>(i: Input<'a, Token>) -> SimpleResult<'a, Token<'a>, AstNode<'a>> {
        parse!{i;
        
            let t = satisfy_with(token_separator_cotent, |sep| sep == Some("("));
            
            let e = expression();
            
            let t = satisfy_with(token_separator_cotent, |sep| sep == Some(")"));
            
            ret e
        }
    }
    fn numer_or_ident<'a>(i: Input<'a, Token>) -> SimpleResult<'a, Token<'a>, AstNode<'a>> {
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

fn term<'a>(i: Input<'a, Token>) -> SimpleResult<'a, Token<'a>, AstNode<'a>> {
    fn sub_term<'a>(i: Input<'a, Token>) -> SimpleResult<'a, Token<'a>, (BiOp, AstNode<'a>)> {
        parse!{i;
            
            let sign = or(mul_sign, div_sign);
            let fa = factor();
            
            ret (sign, fa)
        }
    }
    
    parse!{i;
        let first_factor = factor();
        
        let sub_terms: Vec<(BiOp, AstNode<'a>)> = many(sub_term);
        
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

fn expression<'a>(i: Input<'a, Token>) -> SimpleResult<'a, Token<'a>, AstNode<'a>> {
    fn sub_expression<'a>(i: Input<'a, Token>) -> SimpleResult<'a, Token<'a>, (AstNode<'a>, Sign)> {
        parse!{i;
            
            
            let sign = sign();
            let term = term();
            ret (term, sign)
        }
    }
    
    parse!{i;
        let first_sign = option(sign, Sign::Plus);
        let first_term = term();
    
        let e: Vec<(AstNode<'a>, Sign)> = many(sub_expression);
        
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

fn condition<'a>(i: Input<'a, Token>) -> SimpleResult<'a, Token<'a>, AstNode<'a>> {
    fn odd_expression<'a>(i: Input<'a, Token>) -> SimpleResult<'a, Token<'a>, AstNode<'a>> {
        parse!{i;
            
            
            let _ = satisfy_with(token_keyword_cotent, |sep| sep == Some("ODD"));
            let ex = expression();
            ret ex
        }
    }

    fn composed_expression<'a>(i: Input<'a, Token>) -> SimpleResult<'a, Token<'a>, AstNode<'a>> {
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



fn statement<'a>(i: Input<'a, Token>) -> SimpleResult<'a, Token<'a>, AstNode<'a>> {
    fn assignment<'a>(i: Input<'a, Token>) -> SimpleResult<'a, Token<'a>, AstNode<'a>> {
        parse!{i;
            
            
            let _ = ident();
            let _ = satisfy_with(token_separator_cotent, |sep| sep == Some(":="));
            
            let ex = expression();
            ret ex
        }
    }
    
    fn call<'a>(i: Input<'a, Token>) -> SimpleResult<'a, Token<'a>, AstNode<'a>> {
        parse!{i;
            
            let _ = satisfy_with(token_keyword_cotent, |sep| sep == Some("CALL"));
            
            let ident = ident();
            ret ident
        }
    }
    
    fn question_mark<'a>(i: Input<'a, Token>) -> SimpleResult<'a, Token<'a>, AstNode<'a>> {
        parse!{i;
            
            let _ = satisfy_with(token_separator_cotent, |sep| sep == Some("?"));
            let ident = ident();
            ret ident
        }
    }
    
    fn exclaimation<'a>(i: Input<'a, Token>) -> SimpleResult<'a, Token<'a>, AstNode<'a>> {
        parse!{i;
            let _ = satisfy_with(token_separator_cotent, |sep| sep == Some("!"));
            let ex = expression();
            ret ex
        }
    }
    
    fn begin_end_block<'a>(i: Input<'a, Token>) -> SimpleResult<'a, Token<'a>, AstNode<'a>> {
        parse!{i;
            
            let _ = satisfy_with(token_keyword_cotent, |sep| sep == Some("BEGIN"));
            
            let statements: Vec<AstNode<'a>> = sep_by1(statement, |idx| satisfy_with(idx, token_separator_cotent, |sep| sep == Some(";")));
            
            let _ = satisfy_with(token_keyword_cotent, |sep| sep == Some("END"));
            
            ret AstNode::BeginEnd({
                statements
            })
        }
    }
    
    fn if_then<'a>(i: Input<'a, Token>) -> SimpleResult<'a, Token<'a>, AstNode<'a>> {
        parse!{i;
            
            let _ = satisfy_with(token_keyword_cotent, |sep| sep == Some("IF"));
            let cod = condition();
            let _ = satisfy_with(token_keyword_cotent, |sep| sep == Some("THEN"));
            let st = statement();
            
            ret AstNode::IfThen {
                condition: Box::new(cod),
                statement: Box::new(st)
            }
        }
    }
    
    fn while_do<'a>(i: Input<'a, Token>) -> SimpleResult<'a, Token<'a>, AstNode<'a>> {
        parse!{i;
            
            let _ = satisfy_with(token_keyword_cotent, |sep| sep == Some("WHILE"));
            let cod = condition();
            let _ = satisfy_with(token_keyword_cotent, |sep| sep == Some("DO"));
            let st = statement();
            
            ret AstNode::WhileDo {
                condition: Box::new(cod),
                statement: Box::new(st)
            }
        }
    }
    
    fn all_choices<'a>(i: Input<'a, Token>) -> SimpleResult<'a, Token<'a>, AstNode<'a>> {
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
        
        
        let s = option(all_choices, AstNode::Number(0));
        ret s
    }
}

fn block<'a>(i: Input<'a, Token>) -> SimpleResult<'a, Token<'a>, AstNode<'a>> {
    fn const_declaration<'a>(i: Input<'a, Token>) -> SimpleResult<'a, Token<'a>, AstNode<'a>> {
        fn sub_const_decl<'a>(i: Input<'a, Token>) -> SimpleResult<'a, Token<'a>, AstNode<'a>> {
            parse!{i;
                let ident = ident();
                let _ = satisfy_with(token_separator_cotent, |sep| sep == Some("="));
                let num = number();
                
                ret ident
            }
        }
        
        parse!{i;
            let _ = satisfy_with(token_keyword_cotent, |sep| sep == Some("CONST"));
            
            let subs: Vec<AstNode<'a>> = sep_by1(sub_const_decl, |idx| satisfy_with(idx, token_separator_cotent, |sep| sep == Some(",")));
            let _ = satisfy_with(token_separator_cotent, |sep| sep == Some(";"));
            
            ret subs[0].clone()
        }
    }
    
    fn var_declaration<'a>(i: Input<'a, Token>) -> SimpleResult<'a, Token<'a>, AstNode<'a>> {
        
        parse!{i;
            
            let _ = satisfy_with(token_keyword_cotent, |sep| sep == Some("VAR"));
            let subs: Vec<AstNode<'a>> = sep_by1(ident, |idx| satisfy_with(idx, token_separator_cotent, |sep| sep == Some(",")));
            let _ = satisfy_with(token_separator_cotent, |sep| sep == Some(";"));
            
            ret subs[0].clone()
        }
    }
    
    fn procedure<'a>(i: Input<'a, Token>) -> SimpleResult<'a, Token<'a>, AstNode<'a>> {
        parse!{i;
            let _ = satisfy_with(token_keyword_cotent, |sep| sep == Some("PROCEDURE"));
            let ident = ident();
            let _ = satisfy_with(token_separator_cotent, |sep| sep == Some(";"));
            let block = block();
            let _ = satisfy_with(token_separator_cotent, |sep| sep == Some(";"));
            
            ret block
        }
    }
    
    parse!{i;
        
        let _ = option(const_declaration, AstNode::Number(0));
        let _ = option(var_declaration, AstNode::Number(0));
        let p: Vec<AstNode> = many(procedure);
        let s = statement();
        ret s
    }
}

pub fn program<'a>(i: Input<'a, Token>) -> SimpleResult<'a, Token<'a>, AstNode<'a>> {
    parse!{i;
        
        let block = block();
        let _ = satisfy_with(token_separator_cotent, |sep| sep == Some("."));
        ret block
    }
}