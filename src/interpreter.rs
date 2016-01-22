use parser::*;
use std::collections::HashMap;
use std::io;

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
        let p_map: HashMap<String, &AstNode<'a>> = HashMap::new();
        
        let mut call_stack = vec![(variables, p_map)];
        
        Self::visit_impl(node, &mut call_stack);
    }
    
    fn visit_impl<'b>(node: &'b AstNode<'a>, call_stack: &mut Vec<(HashMap<String, i32>, HashMap<String, &'b AstNode<'a>>)>) -> Option<i32> {
        match *node {
            AstNode::Number(num) => Some(num),
            AstNode::Ident(ref s) => {
                let v = Self::get_var_entry(call_stack, s.to_string());
                Some(*v)
            }
            AstNode::Factor(ref n) => {
                Self::visit_impl(n, call_stack)
            }
            AstNode::Term {ref factors, ref ops} => {
                // println!("factors = {:?}", factors);
                let first_op = [BiOp::Mul];
                let v = factors.iter().map(|f| Self::visit_impl(f, call_stack)).zip(first_op.iter().chain(ops));
                
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
                    (Self::visit_impl(&terms[0], call_stack).unwrap(), terms.iter().skip(1))
                };
                let v = r_terms.map(|f| Self::visit_impl(f, call_stack)).zip(signs);
                
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
                    Self::visit_impl(s, call_stack);
                }
                None
            }
            AstNode::IfThen {ref condition, ref statement} => {
                if Self::evaluate_codition(condition, call_stack) {
                    Self::visit_impl(statement, call_stack);
                }
                None
            }
            AstNode::WhileDo {ref condition, ref statement} => {
                while Self::evaluate_codition(condition, call_stack) {
                    Self::visit_impl(statement, call_stack);
                }
                None
            }
            AstNode::Assignment {ref ident, ref expression} => {
                let ident = Self::get_ident(ident);
                // println!("assign called, ident = {}", ident);
                // println!("ex = {:?}", expression);
                
                let ex_ret = Self::visit_impl(expression, call_stack);
                
                let e = Self::get_var_entry(call_stack, ident);
                
                *e = ex_ret.unwrap();
                
                None
            }
            AstNode::Call(ref ident) => {
                let ident = Self::get_ident(ident);
                
                let p = {
                    let curr_scope = call_stack.last_mut().unwrap();
                    *curr_scope.1.get(&ident).unwrap()
                };
                
                let v_s: HashMap<String, i32> = HashMap::new();
                let p_map: HashMap<String, &AstNode<'a>> = HashMap::new();
                call_stack.push((v_s, p_map));
                
                Self::visit_impl(p, call_stack);
                call_stack.pop();
                
                None
            }
            AstNode::QuestionMark(ref ident) => {
                let mut input_text = String::new();
                io::stdin()
                    .read_line(&mut input_text)
                    .expect("failed to read from stdin");

                let trimmed = input_text.trim();
                match trimmed.parse::<i32>() {
                    Ok(i) => {
                        let ident = Self::get_ident(ident);
                        let e = Self::get_var_entry(call_stack, ident);
                
                        *e = i;
                    }
                    Err(..) => panic!("wrong input"),
                };
                // TODO
                None
            }
            AstNode::ExclaimationMark(ref expression) => {
                let ex_ret = Self::visit_impl(expression, call_stack).unwrap();
                println!("{}", ex_ret);
                None
            }
            AstNode::Const {ref ident, ref value} => {
                // println!("const");
                let curr_scope = call_stack.last_mut().unwrap();
                
                let ident = Self::get_ident(ident);
                let val = Self::get_number(value);
                
                curr_scope.0.insert(ident, val);
                None
            }
            AstNode::Procedure {ref ident, ref block} => {
                let ident = Self::get_ident(ident);
                // println!("inserting pro: {}", ident);
                let curr_scope = call_stack.last_mut().unwrap();
                
                curr_scope.1.insert(ident, block);
                None
            }
            AstNode::Block {ref const_decl, ref var_decl, ref procedures, ref statement} => {
                for c_decl in const_decl {
                    Self::visit_impl(c_decl, call_stack);
                }
                for v_decl in var_decl {
                    let curr_scope = call_stack.last_mut().unwrap();
                
                    let ident = Self::get_ident(v_decl);
                    let val = 0;
                    
                    curr_scope.0.insert(ident, val);
                }
                for p in procedures {
                    Self::visit_impl(p, call_stack);
                }
                Self::visit_impl(statement, call_stack);
                // println!("block");
                None
            }
        }
    }
    
    fn evaluate_codition<'b>(node: &'b AstNode<'a>, call_stack: &mut Vec<(HashMap<String, i32>, HashMap<String, &'b AstNode<'a>>)>) -> bool {
        match *node {
            AstNode::Odd(ref ex) => {
                let r = Self::visit_impl(ex, call_stack).unwrap();
                
                if r % 2 == 0 { false } else { true }
            }
            AstNode::ComposedExpression {ref ex1, ref op, ref ex2} => {
                let ex_ret1 = Self::visit_impl(ex1, call_stack).unwrap();
                let ex_ret2 = Self::visit_impl(ex2, call_stack).unwrap();
                
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
    
    fn get_var_entry<'b>(call_stack: &'b mut Vec<(HashMap<String, i32>, HashMap<String, &AstNode<'a>>)>, var_name: String) -> &'b mut i32 {
        
        for vp in call_stack.iter_mut().rev() {
            //let (v, _):() = vp;
            if let Some(x) = vp.0.get_mut(&var_name) {
                return x;
            }
        }
        panic!("variable not found");
    }
}