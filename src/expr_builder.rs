use std::sync::{Arc, RwLock};

use linked_hash_map::LinkedHashMap;
// use std::{thread, time};

#[derive(Debug)]
pub enum Token {
    VAR(usize),
    LParen,
    RParen,
    OP(Operator)
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Operator {
    AND,
    OR,
    XOR,
    NOT
}

#[derive(Debug)]
pub struct Node {
    val: Token,
    children: Vec<Option<ASTNode>>
}

type ASTNode = Arc<RwLock<Node>>;

#[derive(Debug)]
pub struct Expr {
    tokens: Vec<Token>,
    pub root: Option<ASTNode>
}

pub fn build(input: &String, bool_vars: &LinkedHashMap<char, usize>) -> Result<Expr, String> {
    let tokens = tokenize(input, bool_vars)?;
    let expr: Expr = Expr {
        tokens,
        root: None
    };

    let complete_expr = build_ast(expr);
    Ok(complete_expr)
}

pub fn tokenize(input: &String, bool_vars: &LinkedHashMap<char, usize>) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();

    for c in input.chars() {
        match c {
            ' ' =>          continue,
            '&' =>          tokens.push(Token::OP(Operator::AND)),
            '|' =>          tokens.push(Token::OP(Operator::OR)),
            '^' =>          tokens.push(Token::OP(Operator::XOR)),
            '!' | '~' =>    tokens.push(Token::OP(Operator::NOT)),
            '(' =>          tokens.push(Token::LParen),
            ')' =>          tokens.push(Token::RParen),
            _ => {
                if !bool_vars.contains_key(&c) {
                    return Err(String::from("Found variable not in other expressions"));
                } else {
                    tokens.push(Token::VAR(bool_vars.get(&c).unwrap().clone()))
                }
            }
        }
    }

    Ok(tokens)
}

pub fn build_ast(expr: Expr) -> Expr {
    if expr.tokens.is_empty() { 
        return Expr {
            tokens: expr.tokens,
            root: None
        }
    } 

    let mut op_stack: Vec<&Token> = Vec::new();
    let mut node_stack: Vec<ASTNode> = Vec::new();

    for token in expr.tokens.iter() {
        match token {
            Token::LParen | Token::OP(Operator::NOT) => op_stack.push(token),
            Token::VAR(ndx) => {
                let leaf_node = create_var_node(*ndx);
                node_stack.push(leaf_node);
            },
            Token::RParen => {
                handle_op(&mut op_stack, &mut node_stack);
                op_stack.pop();
            },
            _ => {
                handle_op(&mut op_stack, &mut node_stack);
                op_stack.push(token);
            }
        }
        // dbg!(token);
        // dbg!(&node_stack);
    }

    if !op_stack.is_empty() {
        handle_op(&mut op_stack, &mut node_stack)
    }
   
    return Expr {
        tokens: expr.tokens,
        root: Some(node_stack.pop().expect("Nothing left on node stack to set as root"))
    }
}

fn handle_op(op_stack: &mut Vec<&Token>, node_stack: &mut Vec<ASTNode>) {
    while let Some(Token::OP(o)) = op_stack.last() {
        if *o == Operator::NOT {
            let child = node_stack.pop().expect("No nodes left on stack: triggered by NOT");
            node_stack.push(create_op_node(*o, vec![Some(child)]))
        } else {
            let right_child = node_stack.pop().expect("No nodes left on stack: trig by bin_op");
            let left_child = node_stack.pop().expect("No nodes left on stack: trig by bin_op");
            node_stack.push(create_op_node(*o, vec![Some(left_child), Some(right_child)]));
        }
        op_stack.pop();
    }
}

fn create_var_node(val: usize) -> ASTNode {
    Arc::new(RwLock::new(
        Node {
            val: Token::VAR(val),
            children: vec![]
        }
    ))
}

fn create_op_node(val: Operator, children: Vec<Option<ASTNode>>) -> ASTNode {
    Arc::new(RwLock::new(
        Node {
            val: Token::OP(val),
            children
        }
    ))
}

pub fn evaluate_expr(expr: &Expr, values: &Vec<bool>) -> bool {
    if let Some(node) = &expr.root {
        node.clone().read().unwrap().evaluate(values)
    } else {
        return false;
    }
}

impl Node {
    pub fn evaluate(&self, values: &Vec<bool>) -> bool {
        // thread::sleep(time::Duration::from_millis(5));
        match &self.val {
            Token::VAR(ndx) => return values[*ndx],
            Token::OP(o) => match o {
                Operator::AND => return self.children[0].as_ref().expect("Unexpected leaf node").clone().read().unwrap().evaluate(values) && 
                self.children[1].as_ref().expect("Unexpected leaf node").clone().read().unwrap().evaluate(values),
                Operator::OR => return self.children[0].as_ref().expect("Unexpected leaf node").clone().read().unwrap().evaluate(values) || 
                self.children[1].as_ref().expect("Unexpected leaf node").clone().read().unwrap().evaluate(values),
                Operator::XOR => return self.children[0].as_ref().expect("Unexpected leaf node").clone().read().unwrap().evaluate(values) ^ 
                self.children[1].as_ref().expect("Unexpected leaf node").clone().read().unwrap().evaluate(values),
                Operator::NOT => return !self.children[0].as_ref().expect("Unexpected leaf node").clone().read().unwrap().evaluate(values),
            }
            _ => panic!("Should not occur")
        }
    }
}
