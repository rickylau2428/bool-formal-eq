// Region-based allocation for BDD
// Keep track of # of nodes in BDD; # of nodes with 0 references (dead)
// Come up with ratio (maybe half?) for when region should be re-allocated and things reassigned
// This may also be a good time to consider a BDD re-ordering? (If doing dynamic ordering)

use std::collections::HashMap;
use std::rc::Rc;
use crate::parser::{Operator, SessionInput};

type Edge = isize;

pub struct BDD {
    vertex_lookup: HashMap<Rc<Vertex>, usize>,
    id_lookup: HashMap<usize, Rc<Vertex>>,
    ref_counts: HashMap<usize, usize>,
    // num_dead: isize
    roots: Vec<Edge>,
    computed_cache: HashMap<Expr, Edge>,
    dead_count: usize,
}

#[derive(Eq, PartialEq, Hash)]
pub struct Vertex {
    var: usize,
    lo: Option<Edge>,
    hi: Option<Edge>
}

#[derive(Eq, PartialEq, Hash)]
pub struct Expr {
    op: Operator,
    lhs: Edge,
    rhs: Edge
}

impl BDD {
    pub fn new(input: &SessionInput) -> Self {
        let mut vertex_lookup: HashMap<Rc<Vertex>, usize> = HashMap::with_capacity(50); // Arbitrary
        let mut id_lookup: HashMap<usize, Rc<Vertex>> = HashMap::with_capacity(50); // Arbitrary
        let ref_counts: HashMap<usize, usize> = HashMap::with_capacity(50); // Arbitrary
        let computed_cache: HashMap<Expr, Edge> = HashMap::with_capacity(50); // Arbitrary

        // A terminal node has a variable # 0 and no low or high children
        let terminal_true = Rc::new(Vertex {var: 0, lo: None, hi: None});

        // A terminal node has an ID of 1
        vertex_lookup.insert(Rc::clone(&terminal_true), 1);
        id_lookup.insert(1, terminal_true);

        let roots = Vec::new();

        Self {
            vertex_lookup,
            id_lookup,
            ref_counts,
            roots,
            computed_cache,
            dead_count: 0
        }
    }

    fn add(&mut self, vertex: Rc<Vertex>) -> usize {
        if let Some(ret) = self.vertex_lookup.get(&vertex) {
            return ret.clone();
        } else {
            let id = self.vertex_lookup.len();
            self.vertex_lookup.insert(vertex, id);
            return id
        }

    }

    pub fn make(&mut self, var: usize, lo: Edge, hi: Edge) -> Edge {
        // Prevent creation of vertices with complemented high edge
        let complement_flag: isize = 1;
        if hi < 0 {
            if lo < 0 {
                lo = lo.abs();
            }
            hi = hi.abs();
            complement_flag *= -1;
        }

        if lo == hi {
            return lo;
        }
        let mut tmp = Rc::new(Vertex {var, lo: Some(lo), hi: Some(hi)});
        if let Some(ret) = self.vertex_lookup.get(&tmp) {
            return (*ret).try_into().unwrap();
        } else {
            let id = self.vertex_lookup.len();
            self.vertex_lookup.insert(Rc::clone(&tmp), id);
            self.id_lookup.insert(id, tmp);
            return isize::try_from(id).unwrap() * complement_flag;
        }
    }

    fn apply_helper(&mut self, op: Operator, lhs: isize, rhs: isize, complemented: bool) -> isize {
        let expr = Expr {op, lhs, rhs};
        if self.computed_cache.contains_key(&expr) {
            let id = self.computed_cache.get(&expr).unwrap();
            self.inc_ref(&id.unsigned_abs());
            return *id;

        // The following 4 cases are the special cases lifted straight from Bryant; best to refactor at some point later
        } else if lhs.abs() == 1 && rhs.abs() == 1 { // Both LHS and RHS are leaves
            let lhs_val = get_id_bool(lhs);
            let rhs_val = get_id_bool(rhs);
            if complemented {
                // By De Morgan's Law: ~a && ~b == ~(a || b)
                match op {
                    Operator::AND => get_const_id(!(lhs_val || rhs_val)),
                    Operator::OR => get_const_id(!(lhs_val && rhs_val)),
                    Operator::XOR => get_const_id(lhs_val ^ rhs_val),
                    _ => panic!("Unary operator NOT is passed into bin op apply")
                } 
            } else {
                match op {
                    Operator::AND => get_const_id(lhs_val && rhs_val),
                    Operator::OR => get_const_id(lhs_val || rhs_val),
                    Operator::XOR => get_const_id(lhs_val ^ rhs_val),
                    _ => panic!("Unary operator NOT is passed into bin op apply")
                } 
            }
        } else if lhs.abs() == 1 || rhs.abs() == 1 { // Either only one of LHS or RHS are leaves
            match (op, lhs, rhs) {
                (Operator::AND, -1, _) | (Operator::AND, _, -1) =>  -1,
                (Operator::AND, 1, rhs) => rhs,
                (Operator::AND, lhs, 1) => lhs,
                (Operator::OR, 1, _) | (Operator::OR, _, 1) => 1,
                (Operator::OR, -1, rhs) => rhs,
                (Operator::OR, lhs, -1) => lhs,
                (Operator::XOR, 1, rhs) => -rhs,
                (Operator::XOR, lhs, 1) => -lhs,
                (Operator::XOR, -1, rhs) => rhs,
                (Operator::XOR, lhs, -1) => lhs
            }
        } else if lhs == rhs { // LHS is equivalent to RHS
            match op {
                Operator::AND => lhs,
                Operator::OR => 1,
                Operator::XOR => -1,
                _ => panic!("Unary operator NOT is passed into bin op apply")
            }
        } else if lhs.abs() == rhs.abs() { // LHS and RHS are complements to each other
            match op {
                Operator::AND => -1,
                Operator::OR | Operator::XOR => 1,
                _ => panic!("Unary operator NOT is passed into bin op apply")
            }
        } else { // Not in special case; recursively calculating cofactors
            let lhs_vertex = self.id_lookup.get(&lhs.unsigned_abs()).unwrap();
            let rhs_vertex = self.id_lookup.get(&rhs.unsigned_abs()).unwrap();

            let min_var = if lhs_vertex.var < rhs_vertex.var { lhs_vertex.var } else { rhs_vertex.var };
            self.dec_ref(&min_var);

            return 0;

        }
    }

    pub fn apply(&mut self, op: Operator, lhs: isize, rhs: isize) -> isize {
        // Functions passed into apply are no longer roots
        self.dec_ref(&lhs.unsigned_abs());
        self.dec_ref(&rhs.unsigned_abs());

        let parity = lhs * rhs;
        if parity > 0 {
            return self.apply_helper(op, lhs, rhs, false);
        } else {
            return self.apply_helper(op, lhs, rhs, true);
        }
    }


    fn inc_ref(&mut self, id: &usize) {
        let vertex = self.id_lookup.get(id).unwrap();
        match vertex.var {
            1 => return,
            var => {
                let count = self.ref_counts.get_mut(id).unwrap();
                *count += 1;
                self.inc_ref(&vertex.lo.unwrap().unsigned_abs());
                self.inc_ref(&vertex.hi.unwrap().unsigned_abs());
            }
        }
    }

    fn dec_ref(&mut self, id: &usize) {
        let vertex = self.id_lookup.get(id).unwrap();
        match vertex.var {
            1  => return,
            v => {
                let count = self.ref_counts.get_mut(id).unwrap();
                *count -= 1;
                self.dec_ref(&vertex.lo.unwrap().unsigned_abs());
                self.dec_ref(&vertex.hi.unwrap().unsigned_abs());
                if *count == 0 {
                    self.dead_count += 1
                }
            }
        }
    }

}

fn get_const_id(val: bool) -> isize {
    if val {
        return 1
    } else {
        return -1
    }
}

fn get_id_bool(val: isize) -> bool {
    return val == 1
}

// use crate::parser::*;
// use std::collections::HashSet;

// #[derive(Eq, PartialEq, Hash)]
// pub struct Vertex {
//     id: usize,
//     var: VAL,
//     low: Option<usize>,
//     hi: Option<usize>,
//     refcount: usize
// }

// #[derive(Eq, PartialEq, Hash)]
// pub enum VAL {
//     Var(usize),
//     Sink(bool)
// }

// pub struct BDDRegion {
//     region: HashSet<Vertex>,
//     num_dead: usize,
//     num_vertex: usize
// }


// impl BDDRegion {
//     pub fn new(input: &SessionInput) -> Self {
//         let mut region = HashSet::with_capacity(50); // Arbitrary for now
//         // TODO: find a better way to avoid freeing terminal node (curr: refcount set to 1 to ensure that terminal nodes are never freed)
//         region.insert(Vertex{id: input.bdd_order.len(), var: VAL::Sink(false), low: None, hi: None, refcount: 1}); // Sink node FALSE
//         region.insert(Vertex{id: input.bdd_order.len(), var: VAL::Sink(true), low: None, hi: None, refcount: 1}); // Sink node TRUE 

//         Self {
//             region,
//             num_dead: 0,
//             num_vertex: 2
//         }
//     }

//     pub fn make(&mut self, var: usize, lo: usize, hi: usize) -> usize {
//         if lo == hi {
//             return lo;
//         } else if 

//     }

// }

