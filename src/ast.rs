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

    pub fn BinOpNode(op: Operator, left: ChildNode, right: ChildNode) -> Self {
        Self::new(Token::OP(op), left, right)
    }

    pub fn UnaryOpNode(op: Operator, child: ChildNode) -> Self {
        Self::new(Token::OP(op), child, None)
    }

    pub fn VarNode(name: char) -> Self {
        Self::new(Token::VAR(name), None, None)
    }

    pub fn get_value(&self) -> parser::Token {
        self.value
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
                    let leaf_node = Node::VarNode(*name);
                    node_stack.push(Box::new(leaf_node));
                }
                parser::Token::RParen => {
                    while let Some(Token::OP(o)) = op_stack.pop() {
                        if *o == Operator::NOT {
                            let child = node_stack.pop();
                            node_stack.push(Box::new(Node::UnaryOpNode(*o, child)));
                        } else {
                            let left_child = node_stack.pop();
                            let right_child = node_stack.pop();
                            node_stack.push(Box::new(Node::BinOpNode(*o, left_child, right_child)));
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
            root: Some(Box::new(Node::BinOpNode(Operator::AND,
                                             Some(Box::new(Node::VarNode('b'))), 
                                            Some(Box::new(Node::VarNode('a'))))))
        };
        compare_ast(ExprAst::build(&expr).root, expected.root);
    }

    #[test]
    fn complex_test() {
        let expr = parser::Expr::build_expr(String::from("a ^ (b & ~(c | d))")).unwrap();
        let expected = ExprAst {
            root: 
            Some(Box::new(Node::BinOpNode(Operator::XOR,
                Some(Box::new(Node::BinOpNode(Operator::AND,
                    Some(Box::new(Node::UnaryOpNode(Operator::NOT, 
                        Some(Box::new(Node::BinOpNode(Operator::OR,
                            Some(Box::new(Node::VarNode('d'))),
                           Some(Box::new(Node::VarNode('c'))))))))),
                   Some(Box::new(Node::VarNode('b')))))),
                   Some(Box::new(Node::VarNode('a'))))))
        };
        compare_ast(ExprAst::build(&expr).root, expected.root);
    } 
}