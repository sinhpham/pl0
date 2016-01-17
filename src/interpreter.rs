use parser::*;
use std::collections::HashMap;

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
    
    fn visit(node: &AstNode<'a>) {
        let mut variables: HashMap<String, i32> = HashMap::new();
        let mut var_stack = vec![variables];
        Self::visit_impl(node, &mut var_stack);
    }
    
    fn visit_impl(node: &AstNode<'a>, var_stack: &mut Vec<HashMap<String, i32>>) -> Option<i32> {
        match *node {
            AstNode::Number(num) => Some(num),
            AstNode::Ident(ref s) => {
                let curr_scope = var_stack.last().unwrap();
                
                Some(*curr_scope.get(s.to_owned()).unwrap())
            },
            AstNode::Factor(ref n) => {
                Self::visit_impl(n, var_stack)
            }
            AstNode::Term{ref factors, ref ops} => {
                let first_op = [BiOp::Mul];
                let v = factors.iter().map(|f| Self::visit_impl(f, var_stack)).zip(first_op.iter().chain(ops));
                
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
                    (Self::visit_impl(&terms[0], var_stack).unwrap(), terms.iter().skip(1))
                };
                let v = r_terms.map(|f| Self::visit_impl(f, var_stack)).zip(signs);
                
                let ret = v.fold(first_val, |acc, (val, op)| {
                    match *op {
                        Sign::Plus => acc + val.unwrap(),
                        Sign::Minus => acc - val.unwrap()
                    }
                });
                println!("ex = {:?}", ret);
                Some(ret)
            },
            AstNode::BeginEnd(ref statements) => {
                for s in statements {
                    Self::visit_impl(s, var_stack);
                }
                None
            },
            AstNode::Assignment {ref ident, ref expression} => {
                println!("assign called");
                
                
                
                let ident = Self::get_ident(ident);
                let ex_ret = Self::visit_impl(expression, var_stack);
                
                let e = Self::get_var_entry(var_stack, ident);
                
                *e = ex_ret.unwrap();
                
                None
            }
            AstNode::Block{ref const_decl, ref var_decl, ref procedures, ref statement} => {
                for c_decl in const_decl {
                    Self::visit_impl(c_decl, var_stack);
                }
                for v_decl in var_decl {
                    let curr_scope = var_stack.last_mut().unwrap();
                
                    let ident = Self::get_ident(v_decl);
                    let val = 0;
                    
                    curr_scope.insert(ident, val);
                }
                for p in procedures {
                    Self::visit_impl(p, var_stack);
                }
                Self::visit_impl(statement, var_stack);
                println!("block");
                None
            }
            AstNode::Const{ref ident, ref value} => {
                println!("const");
                let curr_scope = var_stack.last_mut().unwrap();
                
                let ident = Self::get_ident(ident);
                let val = Self::get_number(value);
                
                curr_scope.insert(ident, val);
                None
            }
            AstNode::Procedure {ref ident, ref block} => {
                //Self::visit_var(ident);
                None
            }
            _ => None
        }
    }
    
    // fn visit_var(node: &AstNode<'a>) {
    //     if let &AstNode::Ident(s) = node {
    //         println!("var {:?}", s);
    //     }
    // }
    
    fn get_ident(node: &AstNode<'a>) -> String {
        if let &AstNode::Ident(s) = node {
            s.to_owned()
        } else {
            panic!("asdf");
        }
    }
    
    fn get_number(node: &AstNode<'a>) -> i32 {
        if let &AstNode::Number(n) = node {
            n
        } else {
            panic!("asdf");
        }
    }
    
    fn get_var_entry(var_stack: &mut Vec<HashMap<String, i32>>, var_name: String) -> &mut i32 {
        
        println!("vn: {}", var_name);
        // TODO: search
        let curr_scope = var_stack.last_mut().unwrap();
        
        curr_scope.get_mut(&var_name).unwrap()
    }
}