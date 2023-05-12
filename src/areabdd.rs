// Region-based allocation for BDD
// Keep track of # of nodes in BDD; # of nodes with 0 references (dead)
// Come up with ratio (maybe half?) for when region should be re-allocated and things reassigned
// This may also be a good time to consider a BDD re-ordering? (If doing dynamic ordering)

use std::collections::HashMap;
use std::rc::Rc;
use crate::parser::{Operator, SessionInput};

type Edge = isize;
type ID = isize;

#[derive(Debug)]
pub struct BDD {
    vertex_lookup: HashMap<Rc<Vertex>, ID>,
    id_lookup: HashMap<ID, Rc<Vertex>>,
    ref_counts: HashMap<ID, usize>,
    roots: Vec<Edge>,
    computed_cache: HashMap<Expr, Edge>,
    dead_count: usize,
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Vertex {
    var: isize, // This value should never be < 1 for non-terminal nodes
    lo: Option<Edge>,
    hi: Option<Edge>
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Expr {
    op: Operator,
    lhs: Edge,
    rhs: Edge
}

impl BDD {
    pub fn new() -> Self {
        let mut vertex_lookup: HashMap<Rc<Vertex>, ID> = HashMap::with_capacity(50); // Arbitrary
        let mut id_lookup: HashMap<ID, Rc<Vertex>> = HashMap::with_capacity(50); // Arbitrary
        let ref_counts: HashMap<ID, usize> = HashMap::with_capacity(50); // Arbitrary
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

    pub fn make(&mut self, var: isize, mut lo: Edge, mut hi: Edge) -> Edge {
        if lo == hi {
            return lo;
        }

        // Prevent creation of vertices with complemented high edge
        let mut complement_flag: isize = 1;
        if hi < 0 {
            if lo < 0 {
                lo = lo.abs();
            } else {
                lo = -lo;
            }
            hi = hi.abs();
            complement_flag *= -1;
        }

        assert!(var > 0); // Variables should always be greater than 0
        let tmp = Rc::new(Vertex {var, lo: Some(lo), hi: Some(hi)});
        if let Some(id) = self.vertex_lookup.get(&tmp) {
            let ret = *id;
            inc_ref(self, &ret);
            return ret;
        } else {
            let id = isize::try_from(self.vertex_lookup.len()).unwrap() + 1;
            self.vertex_lookup.insert(Rc::clone(&tmp), id);
            self.id_lookup.insert(id, tmp);
            self.ref_counts.insert(id, 1);
            return id * complement_flag;
        }
    }

    pub fn apply(&mut self, op: Operator, lhs: isize, rhs: isize) -> isize {
        // Functions passed into apply are no longer roots
        dec_ref(self, &lhs);
        dec_ref(self, &rhs);

        let parity = lhs * rhs;
        if parity > 0 {
            return self.apply_helper(op, lhs, rhs, false);
        } else {
            return self.apply_helper(op, lhs, rhs, true);
        }
    }

    fn apply_helper(&mut self, op: Operator, lhs: isize, rhs: isize, mut complemented: bool) -> isize {
        let expr = Expr {op, lhs, rhs};
        if self.computed_cache.contains_key(&expr) {
            let id = *self.computed_cache.get(&expr).unwrap();
            inc_ref(self, &id);
            return id;

        // The following 4 cases are the special cases lifted straight from Bryant; best to refactor at some point later
        } else if lhs.abs() == 1 && rhs.abs() == 1 { // Both LHS and RHS are leaves

            let mut lhs_val = get_id_bool(lhs);
            let mut rhs_val = get_id_bool(rhs);
            if complemented {
                lhs_val = !lhs_val;
                rhs_val = !rhs_val;
            }

            match op {
                Operator::AND => get_const_id(lhs_val && rhs_val),
                Operator::OR => get_const_id(lhs_val || rhs_val),
                Operator::XOR => get_const_id(lhs_val ^ rhs_val),
                _ => panic!("Unary operator NOT is passed into bin op apply")
            } 
            
        } else if lhs.abs() == 1 || rhs.abs() == 1 { // Either only one of LHS or RHS are leaves
            if complemented { // The leaf is the 0-terminal (complemented 1-terminal)
                match (op, lhs, rhs) {
                    (Operator::AND, 1|-1, _) | (Operator::AND, _, 1|-1) => -1,
                    (Operator::OR | Operator::XOR, 1|-1, rhs) => rhs,
                    (Operator::OR | Operator::XOR, lhs, 1|-1) => lhs,
                    _ => panic!("Encountered invalid case in apply")
                }
            } else {
                match (op, lhs, rhs) { // The leaf is the 1-terminal (not complemented)
                    (Operator::AND, 1|-1, rhs) => rhs,
                    (Operator::AND, lhs, 1|-1) => lhs,
                    (Operator::OR, 1|-1, _) | (Operator::OR, _, 1|-1) => 1,
                    (Operator::XOR, 1|-1, rhs) => -rhs,
                    (Operator::XOR, lhs, 1|-1) => -lhs,
                    _ => panic!("Encountered invalid case in apply")
                }
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
            let lhs_vertex = Rc::clone(self.id_lookup.get(&lhs).unwrap());
            let rhs_vertex = Rc::clone(self.id_lookup.get(&rhs).unwrap());

            let lhs_var: isize = lhs_vertex.var;
            let rhs_var: isize = rhs_vertex.var;
            let min_var: isize = if lhs_var < rhs_var { lhs_var } else { rhs_var };

            let (mut lhs_lo, mut lhs_hi) = (lhs_var, lhs_var);
            let (mut rhs_lo, mut rhs_hi) = (rhs_var, rhs_var);
            if lhs_var == min_var {
                (lhs_lo, lhs_hi) = (lhs_vertex.lo.unwrap(), lhs_vertex.hi.unwrap());
                dec_ref(self, &lhs); // dec_ref the root(s) that is being split
            }

            if rhs_var == min_var {
                (rhs_lo, rhs_hi) = (rhs_vertex.lo.unwrap(), rhs_vertex.hi.unwrap());
                dec_ref(self, &rhs); // dec_ref the root(s) that is being split
            }

            let hi_cofactor = self.apply_helper(op, lhs_hi, rhs_hi, complemented);

            if (lhs_lo * rhs_lo) < 0 {
                complemented = !complemented;
            }
            let lo_cofactor = self.apply_helper(op, lhs_lo, rhs_lo, complemented);

            let res = self.make(min_var, lo_cofactor, hi_cofactor);

            self.computed_cache.insert(Expr {op, lhs, rhs}, res);
            return res;

        }
    }

}

impl PartialEq for BDD {
    fn eq(&self, other: &Self) -> bool {
        fn map_eq<K, V>(a: &HashMap<K, V>, b: &HashMap<K,V>) -> bool
        where 
            K: Eq + std::hash::Hash,
            V: Eq
        {
            for (key, val_a) in a.iter() {
                if let Some(val_b) = b.get(key) {
                    if val_a != val_b {
                        return false
                    }
                } else {
                    return false
                }
            }
            return true
        }
        // Compare Vertex Lookup table
       return map_eq(&self.vertex_lookup, &other.vertex_lookup) &&
              map_eq(&self.id_lookup, &other.id_lookup) &&
              map_eq(&self.ref_counts, &other.ref_counts) &&
              map_eq(&self.computed_cache, &other.computed_cache);
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

fn inc_ref (bdd: &mut BDD, id: &isize) {
    let vertex = Rc::clone(bdd.id_lookup.get(id).unwrap());
    match vertex.var {
        0 => return,
        _ => {
            let count = bdd.ref_counts.get_mut(id).unwrap();
            *count += 1;
            inc_ref(bdd, &vertex.lo.unwrap());
            inc_ref(bdd, &vertex.hi.unwrap());
        }
    }
}

fn dec_ref(bdd: &mut BDD, id: &isize) {
    let vertex = Rc::clone(bdd.id_lookup.get(id).unwrap());
    match vertex.var {
        0  => return,
        _ => {
            let count = bdd.ref_counts.get_mut(id).unwrap();
            *count -= 1;

            if *count == 0 {
                bdd.dead_count += 1
            }

            dec_ref(bdd, &vertex.lo.unwrap());
            dec_ref(bdd, &vertex.hi.unwrap());
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn make_node_simple() {
        let mut expected = BDD::new();
        let mut actual = BDD::new();

        let var = 1;
        let lo = 1;
        let hi = -1;

        let expected_node = Rc::new(Vertex {var, lo: Some(-lo), hi: Some(-hi)});
        expected.vertex_lookup.insert(expected_node.clone(), 2);
        expected.id_lookup.insert(2, expected_node);
        expected.ref_counts.insert(2, 1);

        let actual_id = actual.make(var, lo, hi);
        dbg!(&expected);
        dbg!(&actual);
        assert!(-2 == actual_id);
        assert!(expected == actual);
    }

    #[test]
    fn apply_test() {

    }

}