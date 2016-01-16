use parser::*;

pub struct Interpreter<'a> {
    ast: AstNode<'a>,
}

impl<'a> Interpreter<'a> {
    pub fn new(ast: AstNode<'a>) -> Self {
        Interpreter {
            ast: ast
        }
    }
    
    pub fn run(&self) {
        println!("run called");
        Self::visit(&self.ast);
    }
    
    fn visit(node: &AstNode<'a>) -> Option<i32> {
        match *node {
            AstNode::Number(num) => Some(num),
            AstNode::Ident(ref s) => {
                // TODO
                None
            },
            AstNode::Factor(ref n) => {
                Self::visit(n)
            }
            AstNode::Term{ref factors, ref ops} => {
                let first_op = [BiOp::Mul];
                let v = factors.iter().map(|f| Self::visit(f)).zip(first_op.iter().chain(ops));
                
                let ret = v.fold(1, |acc, (val, op)| {
                    match *op {
                        BiOp::Mul => {
                            acc * val.unwrap()
                        },
                        BiOp::Div => {
                            acc / val.unwrap()
                        }
                    }
                });
                
                Some(ret)
            }
            AstNode::Expression{ref terms, ref signs} => {
                let (first_val, r_terms) = if terms.len() == signs.len() {
                    (0, terms.iter().skip(0))
                } else {
                    (Self::visit(&terms[0]).unwrap(), terms.iter().skip(1))
                };
                let v = r_terms.map(|f| Self::visit(f)).zip(signs);
                
                let ret = v.fold(first_val, |acc, (val, op)| {
                    match *op {
                        Sign::Plus => acc + val.unwrap(),
                        Sign::Minus => acc - val.unwrap()
                    }
                });
                
                Some(ret)
            }
            AstNode::Block{ref const_decl, ref var_decl, ref procedures, ref statement} => {
                for c_decl in const_decl {
                    Self::visit(c_decl);
                }
                for v_decl in var_decl {
                    Self::visit_var(v_decl);
                }
                for p in procedures {
                    Self::visit(p);
                }
                println!("block");
                None
            }
            AstNode::Const{ref ident, ref value} => {
                println!("const");
                None
            }
            AstNode::Procedure {ref ident, ref block} => {
                Self::visit_var(ident);
                None
            }
            _ => None
        }
    }
    
    fn visit_var(node: &AstNode<'a>) {
        if let &AstNode::Ident(s) = node {
            println!("var {:?}", s);
        }
    }
}