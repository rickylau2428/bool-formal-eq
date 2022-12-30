use crate::parser;
use crate::parser::{Token, Operator};

type ChildNode = Option<Box<Node>>;

pub struct Node {
    value: Token,
    left: ChildNode,
    right: ChildNode
}

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

    pub fn UnaryOpNode(op: Operator, left: ChildNode) -> Self {
        Self::new(Token::OP(op), left, None)
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
        self.value == self.value
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
                            let child = node_stack.pop().unwrap();
                            node_stack.push(Box::new(Node::UnaryOpNode(o, child)));
                        } else {
                            let leftChild = node_stack.pop().unwrap();
                            let rightChild = node_stack.pop().unwrap();
                            node_stack.push(Box::new(Node::BinOpNode(o, leftChild, rightChild)));
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
    use crate::parser::{Token, Operator};

    fn compare_ast(rootA: &ChildNode, rootB: &ChildNode) -> bool {
        if (rootA == None && rootB == None) {
            true
        }

        if let (Some(nodeA), Some(nodeB)) = (rootA, rootB) {
            (nodeA == nodeB) && compare_ast(nodeA.left, nodeB.left) && compare_ast(nodeA.right, nodeB.right)
        }

        false

    }
    
    #[test]
    fn simple_test() {
        let expr = vec![VAR('a'), OP(AND), VAR('b')];
        let expected = Some(Node::BinOpNode(AND, Node::VarNode('b'), Node::VarNode('a')));
        assert_eq!(ExprAst::build(expr), expected);
    }
}