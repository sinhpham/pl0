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
    
    fn visit(node: &AstNode<'a>) {
        match *node {
            AstNode::Expression{ref terms, ref signs} => {}
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
                
            }
            AstNode::Const{ref ident, ref value} => {
                println!("const");
            }
            AstNode::Procedure {ref ident, ref block} => {
                Self::visit_var(ident);
            }
            _ => {}
        };
        println!("visit");
    }
    
    fn visit_var(node: &AstNode<'a>) {
        if let &AstNode::Ident(s) = node {
            println!("var {:?}", s);
        }
    }
}