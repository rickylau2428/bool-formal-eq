use crate::parser::*;
use std::sync::{Arc, RwLock};

#[derive(Debug, PartialEq)]
pub enum Node {
    OP(OpNode),
    VAR(VarNode)
}

#[derive(Debug, PartialEq)]
struct VarNode {
    val: usize
}

#[derive(Debug)]
struct OpNode {
    op: Operator,
    children: Vec<Option<ASTNode>>
}

type ASTNode = Arc<RwLock<Node>>;

#[derive(Debug)]
pub struct ASTSession {
    pub roots: Vec<ASTNode>
}

impl PartialEq for OpNode {
    fn eq(&self, other: &Self) -> bool {
        for (a, b) in self.children.iter().zip(other.children.iter()) {
            let lhs = a.as_ref().expect("unexpected leaf node").clone();
            let lhs_read = lhs.read().unwrap();
            let rhs = b.as_ref().expect("unexpected leaf node").clone();
            let rhs_read = rhs.read().unwrap();
            if *lhs_read != *rhs_read { return false; }
        }
        return self.op == other.op;
    }
}

impl PartialEq for ASTSession {
    fn eq(&self, other: &Self) -> bool {
        for (a, b) in self.roots.iter().zip(other.roots.iter()) {
            let lhs = a.clone();
            let lhs_read = lhs.read().unwrap();
            let rhs = b.clone();
            let rhs_read = rhs.read().unwrap();
            if *lhs_read != *rhs_read { return false; }
        }
        true
    }
}

pub fn build_ast_session(inputs: &SessionInput) -> ASTSession {
    if inputs.exprs.is_empty() {
        return ASTSession {
            roots: Vec::new()
        }
    }

    let mut roots = Vec::with_capacity(inputs.exprs.len());

    let mut build_ast = |rpn: &Vec<Token>| {
        let mut node_stack = Vec::with_capacity(rpn.len());

        for token in rpn.iter() {
            match token {
                Token::VAR(c) => {
                    let ndx = inputs.ast_order.get(c).expect("Test").clone();
                    node_stack.push(create_var_node(ndx))
                },
                Token::OP(o) => {
                    let right_child = node_stack.pop().expect("No nodes left on stack for binop");
                    let left_child = node_stack.pop().expect("No nodes left on stack for binop");
                    node_stack.push(create_op_node(*o, vec![Some(left_child), Some(right_child)]));
                }
                _ => unreachable!("Match encountered non-op or var token in build_ast")
            }
        }

        let root = node_stack.pop().expect("No nodes left to assign as root in build_ast");
        roots.push(root);
    };

    for expr in inputs.exprs.iter() {
        build_ast(&expr.rpn);
    }

    return ASTSession {
        roots
    }
}

fn create_var_node(val: usize) -> ASTNode {
    Arc::new(RwLock::new(
        Node::VAR(VarNode{
            val
        })
    ))
}

fn create_op_node(op: Operator, children: Vec<Option<ASTNode>>) -> ASTNode {
    Arc::new(RwLock::new(
        Node::OP(OpNode { op, children })))
        }

impl Node {
    pub fn evaluate(&self, values: &Vec<bool>) -> bool {
        // thread::sleep(time::Duration::from_millis(5));
        match &self {
            Node::VAR(node) => return values[node.val],
            Node::OP(node) => match node.op {
                Operator::AND => return node.children[0].as_ref().expect("Unexpected leaf node").clone().read().unwrap().evaluate(values) &&
                node.children[1].as_ref().expect("Unexpected leaf node").clone().read().unwrap().evaluate(values),
                Operator::OR => return node.children[0].as_ref().expect("Unexpected leaf node").clone().read().unwrap().evaluate(values) || 
                node.children[1].as_ref().expect("Unexpected leaf node").clone().read().unwrap().evaluate(values),
                Operator::XOR => return node.children[0].as_ref().expect("Unexpected leaf node").clone().read().unwrap().evaluate(values) ^ 
                node.children[1].as_ref().expect("Unexpected leaf node").clone().read().unwrap().evaluate(values),
                Operator::NOT => return !node.children[0].as_ref().expect("Unexpected leaf node").clone().read().unwrap().evaluate(values),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use linked_hash_map::LinkedHashMap;

    use super::*;
    fn setup() -> SessionInput {
        let exprs = Vec::new();
        let ast_order = LinkedHashMap::new();
        let bdd_order = LinkedHashMap::new();
        SessionInput { exprs, ast_order, bdd_order }
    }

    #[test]
    fn simple_tree() {
        let expr = vec![String::from("a & b")];
        let session = crate::parser::create_session(&expr).unwrap();
        let res = build_ast_session(&session);
        let expected = ASTSession {
            roots: vec![create_op_node(Operator::AND, vec![Some(create_var_node(0)), Some(create_var_node(1))])]
        };
        assert_eq!(expected, res)
    }
}