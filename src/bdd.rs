use std::rc::{Rc, Weak};
use std::collections::HashSet;
use crate::parser::*;

pub struct BDDSession {
    // unique_table: HashMap<BDDVertexInner, usize>
    // unique_table: HashMap<BDDVertex, usize>,
    // node_table: HashMap<usize, BDDVertex>
    unique_table: HashSet<BDDVertex>
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum BDDInner {
    SINK(bool),
    VAR(VarNode)
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct VarNode {
    var: usize,
    lo: BDDVertex,
    hi: BDDVertex
}

type BDDVertex = Rc<BDDInner>;

impl BDDSession {

    pub fn init(input: &SessionInput) -> Self {
        let num_vars = input.bdd_order.len();
        let mut unique_table = HashSet::with_capacity(num_vars);
        unique_table.insert(Rc::new(BDDInner::SINK(false)));
        unique_table.insert(Rc::new(BDDInner::SINK(true)));

        BDDSession { unique_table }
    }

    fn add(&mut self, node: BDDVertex) -> bool {
        self.unique_table.insert(node)
    }

    fn member(&self, node: &BDDVertex) -> bool {
        return self.unique_table.contains(node);
    }

    fn lookup(&self, node: &BDDVertex) -> BDDVertex {
        let ret = self.unique_table.get(node).expect("Node not found in lookup");
        return ret.clone();
    }

    pub fn make(&mut self, var: usize, lo: BDDVertex, hi: BDDVertex) -> BDDVertex {
        if lo == hi {
            return lo.clone();
        }

        let vertex = Rc::new(BDDInner::VAR(VarNode {var, lo, hi}));
        if self.member(&vertex) {
            return self.lookup(&vertex);
        } else {
            let ret = vertex.clone();
            self.add(vertex);
            return ret;
        }
    }

    pub fn build(&self, bool_expr: &Vec<Token>, var_count: usize) {

    }
}


// pub fn build(bool_expr: Vec<Token>, var_count: usize, session: &BDDSession) {
//     build_helper = |ndx: usize, bool_expr: Vec<Token>| -> BDDVertex {
//         if ndx > var_count {
//             return eval_expr(bool_expr).unwrap();
//         } else {
//             let lo = build_helper()
//         }

//     };

// }

fn eval_expr(expr: Vec<Token>) -> Result<bool, &'static str> {
    let mut eval_stack = Vec::new();

    for token in expr.iter() {
        match token {
            Token::VAL(b) => eval_stack.push(b.clone()),
            Token::OP(o) => {
                if let Operator::NOT = o {
                    let val = eval_stack.pop().expect("NOT without val in eval_stack");
                    eval_stack.push(!val);
                }

                let right_val = eval_stack.pop().expect("Bin op with 0 arguments");
                let left_val = eval_stack.pop().expect("Bin op with only 1 argument");

                match o {
                    Operator::AND => eval_stack.push(right_val & left_val),
                    Operator::OR => eval_stack.push(right_val | left_val),
                    Operator::XOR => eval_stack.push(right_val ^ left_val),
                    _ => return Err("Non-operator token found")
                }
            },
            _ => return Err("Variable found in evaluation")
        }
    }

    if eval_stack.len() != 1 {
        return Err("Unbalanced evaluation of expr")
    }

   Ok(eval_stack.pop().unwrap())
}

fn sub_constant(bool_expr: &Vec<Token>, var: char, sub: bool) {
    let mut out = bool_expr.clone();
    for token in out.iter_mut() {
        match token {
            Token::VAR(c) => {
                if *c == var {
                    *token = Token::VAL(sub);
                }
            },
            _ => {}
        }
    }
}

