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

    pub fn build(&self, bool_expr: Vec<Token>, var_count: usize) {

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

// fn eval_expr(expr: Vec<Token>) -> Result<BDDVertex, &'static str> {
//     Err("stub")
// }

// fn sub_constant(bool_expr: &Vec<Token>, ndx: usize, val: bool) {
//     let out = bool_expr.clone();
//     for token in out.iter() {
//         match token {
//             Token::VAR(c) => {
                
//             }
//         }
//     }

// }

