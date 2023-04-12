use std::rc::{Rc};
use std::collections::HashSet;
use crate::parser::*;

#[derive(Debug)]
pub struct BDDSession {
    // unique_table: HashMap<BDDVertexInner, usize>
    // unique_table: HashMap<BDDVertex, usize>,
    // node_table: HashMap<usize, BDDVertex>
    unique_table: HashSet<BDDVertex>,
    roots: Vec<BDDVertex>
    // computed_table
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum BDDInner {
    SINK(bool),
    VAR(VarNode)
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
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
        let roots = Vec::with_capacity(num_vars);
        unique_table.insert(Rc::new(BDDInner::SINK(false)));
        unique_table.insert(Rc::new(BDDInner::SINK(true)));

        let mut ret = BDDSession { unique_table, roots };
        for expr in input.exprs.iter() {
            ret.build(expr.rpn.clone(), &input.bdd_order);
        }

        ret
    }

    fn init_test(num_vars: usize) -> Self {
        let mut unique_table = HashSet::with_capacity(num_vars);
        let roots = Vec::with_capacity(num_vars);
        unique_table.insert(Rc::new(BDDInner::SINK(false)));
        unique_table.insert(Rc::new(BDDInner::SINK(true)));

        BDDSession { unique_table, roots }
    }

    fn add(&mut self, node: BDDVertex) -> bool {
        self.unique_table.insert(node)
    }

    fn member(&self, node: &BDDVertex) -> bool {
        return self.unique_table.contains(node);
    }

    fn lookup(&self, node: BDDVertex) -> BDDVertex {
        let ret = self.unique_table.get(&node).expect("Node not found in lookup");
        return ret.clone();
    }

    pub fn make(&mut self, var: usize, lo: BDDVertex, hi: BDDVertex) -> BDDVertex {
        if lo == hi {
            return lo.clone();
        }

        let vertex = Rc::new(BDDInner::VAR(VarNode {var, lo, hi}));
        if self.member(&vertex) {
            return self.lookup(vertex);
        } else {
            let ret = vertex.clone();
            self.add(vertex);
            return ret;
        }
    }

    pub fn build(&mut self, bool_expr: Vec<Token>, ordering: &Vec<char>) {
        let root = self.build_helper(bool_expr, 1, ordering);
        self.roots.push(root);
    }

    fn build_helper(&mut self, expr: Vec<Token>, ndx: usize, ordering: &Vec<char>) -> BDDVertex {
        if ndx > ordering.len() {
            let bool_res = eval_expr(expr).expect("Evaluation failed");
            self.lookup(Rc::new(BDDInner::SINK(bool_res))) 
        } else {
            let lo = self.build_helper(sub_constant(&expr, ordering[ndx-1], false), ndx+1, ordering);
            let hi = self.build_helper(sub_constant(&expr, ordering[ndx-1], true), ndx+1, ordering);
            return self.make(ndx, lo, hi)
        }
    }
}


fn eval_expr(expr: Vec<Token>) -> Result<bool, &'static str> {
    let mut eval_stack = Vec::new();

    for token in expr.iter() {
        match token {
            Token::VAL(b) => eval_stack.push(b.clone()),
            Token::OP(o) => {
                if let Operator::NOT = o {
                    let val = eval_stack.pop().expect("NOT without val in eval_stack");
                    eval_stack.push(!val);
                    continue;
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

fn sub_constant(bool_expr: &Vec<Token>, var: char, sub: bool) -> Vec<Token> {
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

    out
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_one_var() {
        let ordering = vec!['a'];
        let rpn = vec![Token::VAR('a')];
        let mut bdd = BDDSession::init_test(1);
        bdd.build(rpn, &ordering);
    }

    #[test]
    fn bin_op() {
        let ordering = vec!['a', 'b'];
        let rpn = vec![Token::VAR('b'), Token::VAR('a'), Token::OP(Operator::OR)];
        let mut bdd = BDDSession::init_test(2);
        bdd.build(rpn, &ordering);
        // dbg!(bdd);
    }

    #[test]
    fn equiv() {
        let ordering = vec!['a', 'b'];
        let rpn1 = vec![Token::VAR('b'), Token::VAR('a'), Token::OP(Operator::OR), Token::OP(Operator::NOT)];
        let rpn2 = vec![Token::VAR('b'), Token::OP(Operator::NOT), Token::VAR('a'), Token::OP(Operator::NOT), Token::OP(Operator::AND)];
        let mut bdd = BDDSession::init_test(2);
        bdd.build(rpn1, &ordering);
        dbg!(&bdd);
        bdd.build(rpn2, &ordering);
        dbg!(&bdd);
        assert_eq!(bdd.roots[0], bdd.roots[1])
    }

}

