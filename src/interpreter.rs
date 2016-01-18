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
        // println!("run called");
        Self::visit(&self.ast);
    }
    
    fn visit(node: &AstNode<'a>) {
        let variables: HashMap<String, i32> = HashMap::new();
        let mut var_stack = vec![variables];
        let mut p_map = HashMap::new();
        
        Self::visit_impl(node, &mut var_stack, &mut p_map);
    }
    
    fn visit_impl<'b>(node: &'b AstNode<'a>, var_stack: &mut Vec<HashMap<String, i32>>, p_map: &mut HashMap<String, &'b AstNode<'a>>) -> Option<i32> {
        match *node {
            AstNode::Number(num) => Some(num),
            AstNode::Ident(ref s) => {
                // println!("Ident = {:?}", s);
                let curr_scope = var_stack.last().unwrap();
                // println!("curr_scope: {:?}", curr_scope);
                Some(*curr_scope.get(s.to_owned()).unwrap())
            }
            AstNode::Factor(ref n) => {
                Self::visit_impl(n, var_stack, p_map)
            }
            AstNode::Term {ref factors, ref ops} => {
                // println!("factors = {:?}", factors);
                let first_op = [BiOp::Mul];
                let v = factors.iter().map(|f| Self::visit_impl(f, var_stack, p_map)).zip(first_op.iter().chain(ops));
                
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
            AstNode::Expression {ref terms, ref signs} => {
                let (first_val, r_terms) = if terms.len() == signs.len() {
                    (0, terms.iter().skip(0))
                } else {
                    (Self::visit_impl(&terms[0], var_stack, p_map).unwrap(), terms.iter().skip(1))
                };
                let v = r_terms.map(|f| Self::visit_impl(f, var_stack, p_map)).zip(signs);
                
                let ret = v.fold(first_val, |acc, (val, op)| {
                    match *op {
                        Sign::Plus => acc + val.unwrap(),
                        Sign::Minus => acc - val.unwrap()
                    }
                });
                // println!("ex = {:?}", ret);
                Some(ret)
            }
            AstNode::Odd(_) => {
                None
            }
            AstNode::ComposedExpression {..} => {
                None
            }
            AstNode::BeginEnd(ref statements) => {
                for s in statements {
                    Self::visit_impl(s, var_stack, p_map);
                }
                None
            }
            AstNode::IfThen {ref condition, ref statement} => {
                if Self::evaluate_codition(condition, var_stack, p_map) {
                    Self::visit_impl(statement, var_stack, p_map);
                }
                None
            }
            AstNode::WhileDo {ref condition, ref statement} => {
                while Self::evaluate_codition(condition, var_stack, p_map) {
                    Self::visit_impl(statement, var_stack, p_map);
                }
                None
            }
            AstNode::Assignment {ref ident, ref expression} => {
                let ident = Self::get_ident(ident);
                // println!("assign called, ident = {}", ident);
                // println!("ex = {:?}", expression);
                
                let ex_ret = Self::visit_impl(expression, var_stack, p_map);
                
                let e = Self::get_var_entry(var_stack, ident);
                
                *e = ex_ret.unwrap();
                
                None
            }
            AstNode::Call(ref ident) => {
                let ident = Self::get_ident(ident);
                
                let p = *p_map.get(&ident).unwrap();
                Self::visit_impl(p, var_stack, p_map);
                None
            }
            AstNode::QuestionMark(_) => {
                // TODO
                None
            }
            AstNode::ExclaimationMark(ref expression) => {
                let ex_ret = Self::visit_impl(expression, var_stack, p_map).unwrap();
                println!("{}", ex_ret);
                None
            }
            AstNode::Const {ref ident, ref value} => {
                // println!("const");
                let curr_scope = var_stack.last_mut().unwrap();
                
                let ident = Self::get_ident(ident);
                let val = Self::get_number(value);
                
                curr_scope.insert(ident, val);
                None
            }
            AstNode::Procedure {ref ident, ref block} => {
                let ident = Self::get_ident(ident);
                // println!("inserting pro: {}", ident);
                p_map.insert(ident, block);
                None
            }
            AstNode::Block {ref const_decl, ref var_decl, ref procedures, ref statement} => {
                for c_decl in const_decl {
                    Self::visit_impl(c_decl, var_stack, p_map);
                }
                for v_decl in var_decl {
                    let curr_scope = var_stack.last_mut().unwrap();
                
                    let ident = Self::get_ident(v_decl);
                    let val = 0;
                    
                    curr_scope.insert(ident, val);
                }
                for p in procedures {
                    Self::visit_impl(p, var_stack, p_map);
                }
                Self::visit_impl(statement, var_stack, p_map);
                // println!("block");
                None
            }
        }
    }
    
    fn evaluate_codition<'b>(node: &'b AstNode<'a>, var_stack: &mut Vec<HashMap<String, i32>>, p_map: &mut HashMap<String, &'b AstNode<'a>>) -> bool {
        match *node {
            AstNode::Odd(ref ex) => {
                let r = Self::visit_impl(ex, var_stack, p_map).unwrap();
                
                if r % 2 == 0 { false } else { true }
            }
            AstNode::ComposedExpression {ref ex1, ref op, ref ex2} => {
                let ex_ret1 = Self::visit_impl(ex1, var_stack, p_map).unwrap();
                let ex_ret2 = Self::visit_impl(ex2, var_stack, p_map).unwrap();
                
                match *op {
                    ExOp::Equal => ex_ret1 == ex_ret2,
                    ExOp::NumberSign => ex_ret1 != ex_ret2,
                    ExOp::LessThan => ex_ret1 < ex_ret2,
                    ExOp::LessThanOrEqual => ex_ret1 <= ex_ret2,
                    ExOp::GreaterThan => ex_ret1 > ex_ret2,
                    ExOp::GreaterThanOrEqual => ex_ret1 >= ex_ret2,
                }
            },
            _ => panic!("invalid condition")
        }
    }
    
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
        
        // println!("vn: {}", var_name);
        // TODO: search
        let curr_scope = var_stack.last_mut().unwrap();
        
        curr_scope.get_mut(&var_name).unwrap()
    }
}