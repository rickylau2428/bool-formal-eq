use crate::parser;
use crate::parser::{Token, Operator};

type ChildNode = Option<Box<Node>>;

#[derive(Debug)]
pub struct Node {
    value: Token,
    left: ChildNode,
    right: ChildNode
}

#[derive(Debug)]
pub struct ExprAst {
    pub root: ChildNode
}

impl Node {
    fn new(value: Token, left: ChildNode, right: ChildNode) -> Node {
        Node {
            value,
            left,
            right
        }
    }

    pub fn bin_op_node(op: Operator, left: ChildNode, right: ChildNode) -> Self {
        Self::new(Token::OP(op), left, right)
    }

    pub fn unary_op_node(op: Operator, child: ChildNode) -> Self {
        Self::new(Token::OP(op), child, None)
    }

    pub fn var_node(ndx: usize) -> Self {
        Self::new(Token::VAR(ndx), None, None)
    }

    pub fn get_value(&self) -> parser::Token {
        self.value
    }

    fn evaluate(&self, values: &Vec<bool>) -> bool {
        match self.value {
            Token::VAR(ndx) => return values[ndx],
            Token::OP(o) => match o {
                Operator::AND => return self.left.as_ref().expect("Left operand empty").evaluate(values) && self.right.as_ref().expect("Right operand empty").evaluate(values),
                Operator::OR => return self.left.as_ref().expect("Left operand empty").evaluate(values) || self.right.as_ref().expect("Right operand empty").evaluate(values),
                Operator::XOR => return self.left.as_ref().expect("left operand empty").evaluate(values) ^ self.right.as_ref().expect("Right operand empty").evaluate(values),
                Operator::NOT => return !self.left.as_ref().expect("child empty").evaluate(values),
            },
            _ => panic!("Should not occur")
        }
    }

}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.left.eq(&other.left) && self.right.eq(&other.right)
    }
}

impl ExprAst {
    // TODO: Better error behaviour
    pub fn build(expression: &parser::Expr) -> ExprAst {

        let mut op_stack: Vec<&parser::Token> = Vec::new(); 
        let mut node_stack: Vec<Box<Node>> = Vec::new();

        for token in expression.tokens.iter() {
            match token {
                parser::Token::VAR(name) => {
                    let leaf_node = Node::var_node(*name);
                    node_stack.push(Box::new(leaf_node));
                }
                parser::Token::RParen => {
                    while let Some(Token::OP(o)) = op_stack.pop() {
                        if *o == Operator::NOT {
                            let child = node_stack.pop();
                            node_stack.push(Box::new(Node::unary_op_node(*o, child)));
                        } else {
                            let left_child = node_stack.pop();
                            let right_child = node_stack.pop();
                            node_stack.push(Box::new(Node::bin_op_node(*o, left_child, right_child)));
                        }
                    }
                }
                _ => op_stack.push(token)
            }
        }

        let root = node_stack.pop();
        ExprAst {
            root
        }
    }

    pub fn evaluate(&self, values: &Vec<bool>) -> bool {
        if let Some(root) = &self.root {
            return root.evaluate(values)
        } else {
            return false;
        }
    }

}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser;

    fn compare_ast(root_a: ChildNode, root_b: ChildNode) -> bool {
        if root_a == None && root_b == None {
           return true
        }

        if let (Some(node_a), Some(node_b)) = (root_a, root_b) {
            return (node_a == node_b) && compare_ast(node_a.left, node_b.left) && compare_ast(node_a.right, node_b.right)
        }

        false
    }
    
    #[test]
    fn simple_test() {
        let expr = parser::Expr::build_expr(String::from("a & b")).unwrap();
        let expected = ExprAst {
            root: Some(Box::new(Node::bin_op_node(Operator::AND,
                                             Some(Box::new(Node::var_node(1))), 
                                            Some(Box::new(Node::var_node(0))))))
        };
        compare_ast(ExprAst::build(&expr).root, expected.root);
    }

    #[test]
    fn complex_test() {
        let expr = parser::Expr::build_expr(String::from("a ^ (b & ~(c | d))")).unwrap();
        let expected = ExprAst {
            root: 
            Some(Box::new(Node::bin_op_node(Operator::XOR,
                Some(Box::new(Node::bin_op_node(Operator::AND,
                    Some(Box::new(Node::unary_op_node(Operator::NOT, 
                        Some(Box::new(Node::bin_op_node(Operator::OR,
                            Some(Box::new(Node::var_node(3))),
                           Some(Box::new(Node::var_node(2))))))))),
                   Some(Box::new(Node::var_node(1)))))),
               Some(Box::new(Node::var_node(0))))))
        };
        compare_ast(ExprAst::build(&expr).root, expected.root);
    } 

    #[test]
    fn empty_eval_test() {
        let test_tree = ExprAst {
            root: None
        };
        assert!(!test_tree.evaluate(&vec![true, true, false]))
    } 
    
    #[test]
    fn single_eval_test() {
        let test_tree = ExprAst {
            root: Some(Box::new(Node::var_node(1)))
        };
        assert!(test_tree.evaluate(&vec![false, true, false]))
    }

    #[test]
    fn simple_tree_eval_and() {
        let test_tree = ExprAst {
            root: Some(Box::new(Node::bin_op_node(Operator::AND,
                Some(Box::new(Node::var_node(0))),
               Some(Box::new(Node::var_node(1))))))
        };
        assert!(test_tree.evaluate(&vec![true, true]))
    }
    
    #[test]
    fn simple_tree_eval_or() {
        let test_tree = ExprAst {
            root: Some(Box::new(Node::bin_op_node(Operator::OR,
                Some(Box::new(Node::var_node(0))),
               Some(Box::new(Node::var_node(1))))))
        };
        assert!(test_tree.evaluate(&vec![false, true]))
    }

    #[test]
    fn simple_tree_eval_xor() {
        let test_tree = ExprAst {
            root: Some(Box::new(Node::bin_op_node(Operator::XOR,
                Some(Box::new(Node::var_node(0))),
               Some(Box::new(Node::var_node(1))))))
        };
        assert!(!test_tree.evaluate(&vec![true, true]))
    }

    #[test]
    fn complex_tree_eval() {
        let expr = parser::Expr::build_expr(String::from("(a & (b ^ ~(a | c))) & b")).unwrap();
        let test_tree = ExprAst::build(&expr);
        assert!(test_tree.evaluate(&vec![true, true, false]))
    }
}