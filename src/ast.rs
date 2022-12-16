use crate::parser;

type ChildNode = Option<Box<Node>>;

pub struct Node {
    pub value: parser::Token,
    left: ChildNode,
    right: ChildNode
}

impl Node {
    pub fn new(value: parser::Token, left: ChildNode, right: ChildNode) -> Node {
        Node {
            value,
            left,
            right
        }
    }

    pub fn get_value(&self) -> parser::Token {
        self.value
    }

    pub fn build(expression: &parser::Expr) -> Node {

        let mut op_stack: Vec<&parser::Token> = Vec::new(); 
        let mut node_stack: Vec<Node> = Vec::new();

        for token in expression.tokens.iter() {
            match token {
                parser::Token::VAR(_) => {
                    let leaf_node = Self::new(*token, None, None);
                    node_stack.push(leaf_node);
                }
                parser::Token::RParen => {
                    

                }
                _ => op_stack.push(token)
            }
        }

        Node {};
    }
}

