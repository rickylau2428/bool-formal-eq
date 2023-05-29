// Region-based allocation for BDD
// Keep track of # of nodes in BDD; # of nodes with 0 references (dead)
// Come up with ratio (maybe half?) for when region should be re-allocated and things reassigned
// This may also be a good time to consider a BDD re-ordering? (If doing dynamic ordering)

use linked_hash_map::LinkedHashMap;
use std::collections::{HashMap};
use std::rc::Rc;
use crate::parser::{Operator, Parser, Token};

type Edge = isize;
type ID = isize;

#[derive(Debug, Clone)]
pub struct BDD {
    vertex_lookup: HashMap<Rc<Vertex>, ID>,
    id_lookup: HashMap<ID, Rc<Vertex>>,
    ref_counts: HashMap<ID, usize>,
    roots: Vec<Edge>,
    computed_cache: HashMap<Expr, Edge>,
    dead_count: usize,
    ordering: HashMap<char, usize>
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub struct Vertex {
    var: isize, // This value should never be < 1 for non-terminal nodes
    lo: Option<Edge>,
    hi: Option<Edge>
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
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
        let ordering: HashMap<char, usize> = HashMap::with_capacity(5); // Arbitrary

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
            dead_count: 0,
            ordering
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
              map_eq(&self.computed_cache, &other.computed_cache) &&
              self.dead_count == other.dead_count;
    }

}

// Adds a variable with the given name into the BDD
// Places it last in the current ordering
pub fn add_var(bdd: &mut BDD, var: char) {
    if let Some(_) = bdd.ordering.get(&var) {
        panic!("Variable already exists in BDD"); // Change to resolve gracefully
    } else {
        let var_id = bdd.ordering.len();
        bdd.ordering.insert(var, var_id);
        let res = make(bdd,  var_id as isize, -1, 1);
        bdd.roots.push(res);
    }
}

fn make(bdd: &mut BDD, var: isize, mut lo: Edge, mut hi: Edge) -> Edge {
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
    if let Some(id) = bdd.vertex_lookup.get(&tmp) {
        let ret = *id;
        inc_ref(bdd, &ret);
        return ret;
    } else {
        let id = isize::try_from(bdd.vertex_lookup.len()).unwrap() + 1;
        bdd.vertex_lookup.insert(Rc::clone(&tmp), id);
        bdd.id_lookup.insert(id, tmp);
        bdd.ref_counts.insert(id, 1);
        return id * complement_flag;
    }
}


pub fn apply(bdd: &mut BDD, op: &Operator, lhs: isize, rhs: isize) -> isize { // Functions passed into apply are no longer roots
    // dec_ref(self, &lhs);
    // dec_ref(self, &rhs);

    let parity = lhs * rhs;
    if parity > 0 {
        return apply_helper(bdd, op, lhs, rhs, false);
    } else {
        return apply_helper(bdd, op, lhs, rhs, true);
    }
}

fn apply_helper(bdd: &mut BDD, op: &Operator, lhs: isize, rhs: isize, mut complemented: bool) -> isize {

    let expr = Expr {op: *op, lhs, rhs}; if bdd.computed_cache.contains_key(&expr) {
        let id = *bdd.computed_cache.get(&expr).unwrap();
        inc_ref(bdd, &id);
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
            // dbg!(&lhs);
            // dbg!(&rhs);
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
        let lhs_vertex = Rc::clone(bdd.id_lookup.get(&lhs.abs()).unwrap());
        let rhs_vertex = Rc::clone(bdd.id_lookup.get(&rhs.abs()).unwrap());

        let lhs_var: isize = lhs_vertex.var;
        let rhs_var: isize = rhs_vertex.var;
        let min_var: isize = if lhs_var < rhs_var { lhs_var } else { rhs_var };

        let (mut lhs_lo, mut lhs_hi) = (lhs, lhs);
        let (mut rhs_lo, mut rhs_hi) = (rhs, rhs);
        if lhs_var == min_var {
            (lhs_lo, lhs_hi) = (lhs_vertex.lo.unwrap(), lhs_vertex.hi.unwrap());
            dec_ref(bdd, &lhs); // dec_ref the root(s) that is being split
        }

        if rhs_var == min_var {
            (rhs_lo, rhs_hi) = (rhs_vertex.lo.unwrap(), rhs_vertex.hi.unwrap());
            dec_ref(bdd, &rhs); // dec_ref the root(s) that is being split
        }

        let hi_cofactor = apply_helper(bdd, op, lhs_hi, rhs_hi, complemented);

        if (lhs_lo * rhs_lo) < 0 {
            complemented = !complemented;
        }
        let lo_cofactor = apply_helper(bdd, op, lhs_lo, rhs_lo, complemented);

        let res = make(bdd, min_var, lo_cofactor, hi_cofactor);

        bdd.computed_cache.insert(Expr {op: *op, lhs, rhs}, res);
        return res;
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
    let abs_id = id.abs();

    let vertex = Rc::clone(bdd.id_lookup.get(&abs_id).unwrap());
    match vertex.var {
        0 => return,
        _ => {
            let count = bdd.ref_counts.get_mut(&abs_id).unwrap();
            *count += 1;
            inc_ref(bdd, &vertex.lo.unwrap());
            inc_ref(bdd, &vertex.hi.unwrap());
        }
    }
}

fn dec_ref(bdd: &mut BDD, id: &isize) {
    let abs_id = id.abs();

    let vertex = Rc::clone(bdd.id_lookup.get(&abs_id).unwrap());
    match vertex.var {
        0  => return,
        _ => {
            let count = bdd.ref_counts.get_mut(&abs_id).unwrap();
            *count -= 1;

            if *count == 0 {
                bdd.dead_count += 1
            }

            dec_ref(bdd, &vertex.lo.unwrap());
            dec_ref(bdd, &vertex.hi.unwrap());
        }
    }
}

// build takes as input a Parser with multiple Boolean expressions. 
// TODO: For the time being, it uses the ast_order to determine variable ordering
// The output is a BDD with the target equations built into it
pub fn build(mut bdd: BDD, parser: &Parser) -> BDD {
    for e in parser.exprs.iter() {
        bdd = build_helper(bdd, &e.rpn, &parser.ast_order);
    }

    return bdd;
}

fn build_helper(mut bdd: BDD, eq: &Vec<Token>, order_map: &LinkedHashMap<char, usize>) -> BDD {
    let mut op_stack: Vec<isize> = Vec::new();

    for t in eq.iter() {
        match t {
            Token::VAR(c) => {
                let var_num = order_map.get(c).unwrap().clone();
                assert!(var_num > 0);
                let node_id = make(&mut bdd, var_num as isize, -1, 1);
                op_stack.push(node_id);
            },
            Token::OP(op) => {
                if *op == Operator::NOT {
                    let top = op_stack.get_mut(0).unwrap();
                    *top = -(*top);
                } else {
                    let rhs = op_stack.pop().unwrap();
                    let lhs = op_stack.pop().unwrap();
                    let res = apply(&mut bdd, op, lhs, rhs);
                    dbg!(res);
                    op_stack.push(res);
                }
            },
            _ => panic!("Unexpected token while building BDD")

        }
    }

    // TODO: What if there are multiple nodes left on the stack..?
    if op_stack.len() == 1 {
        let root = op_stack.pop().unwrap();
        bdd.roots.push(root);
    } else {
        panic!("No vertex left to assign as root");
    }

    return bdd;
}

pub fn satisfy_count(bdd: &BDD, root: isize) -> usize {


        

    return 0;
}

fn count_helper(bdd: &BDD, vertex_id: isize, mut complemented: bool) -> usize {
    if vertex_id.abs() == 1 {
        if complemented { return 1; } else { return 0; }
    } else {
        let vertex = bdd.id_lookup.get(&vertex_id.abs()).unwrap();
        let lo_var = bdd.id_lookup.get(&vertex.lo.unwrap()).unwrap().var;
        let hi_var = bdd.id_lookup.get(&vertex.hi.unwrap()).unwrap().var;
        let mut lo_sat: usize = 2usize.pow(u32::try_from(lo_var).unwrap() - u32::try_from(vertex.var).unwrap() - 1);
        let mut hi_sat: usize = 2usize.pow(u32::try_from(hi_var).unwrap() - u32::try_from(vertex.var).unwrap() - 1);

        if vertex.lo.unwrap().abs() == 1 {
            lo_sat = 2usize.pow(bdd.ordering.len() as u32 - u32::try_from(vertex.var).unwrap());
        }

        if vertex.hi.unwrap().abs() == 1 {
            hi_sat = 2usize.pow(bdd.ordering.len() as u32 - u32::try_from(vertex.var).unwrap());
        }

        let hi_count = hi_sat * count_helper(bdd, vertex.hi.unwrap(), complemented);

        if vertex.lo.unwrap() * vertex.hi.unwrap() < 0 {
            complemented = !complemented;
        }

        let lo_count = lo_sat * count_helper(bdd, vertex.hi.unwrap(), complemented);

        return lo_count + hi_count;

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

        let actual_id = make(&mut actual, var, lo, hi);
        dbg!(&expected);
        dbg!(&actual);
        assert!(-2 == actual_id);
        assert!(expected == actual);
    }

    #[test]
    fn apply_test() {
        let mut actual = BDD::new();

        make(&mut actual, 1, -1, 1);
        make(&mut actual, 2, -1, 1);
        // dbg!(&actual);
        let mut expected = actual.clone();

        let expected_node = Rc::new(Vertex {var:1, lo: Some(-1), hi: Some(3)});
        expected.vertex_lookup.insert(expected_node.clone(), 4);
        expected.id_lookup.insert(4, expected_node);
        expected.computed_cache.insert(Expr {op: Operator::AND, lhs: 2, rhs: 3}, 4);
        let rc = expected.ref_counts.get_mut(&2).unwrap();
        *rc = 0;
        expected.ref_counts.insert(4, 1);
        expected.dead_count = 1;

        let actual_id = apply(&mut actual, &Operator::AND, 2, 3);
        dbg!(&expected.vertex_lookup);
        dbg!(&actual.vertex_lookup);
        assert!(expected == actual);
        assert!(4 == actual_id);
    }

    #[test]
    fn apply_xor() {
        let mut actual = BDD::new();

        make(&mut actual, 1, -1, 1);
        make(&mut actual, 2, -1, 1);
        // dbg!(&actual);
        let mut expected = actual.clone();

        let expected_node = Rc::new(Vertex {var:1, lo: Some(-3), hi: Some(3)});
        expected.vertex_lookup.insert(expected_node.clone(), 4);
        expected.id_lookup.insert(4, expected_node);
        expected.computed_cache.insert(Expr {op: Operator::XOR, lhs: -2, rhs: 3}, 4);
        let rc = expected.ref_counts.get_mut(&2).unwrap();
        *rc = 0;
        expected.ref_counts.insert(4, 1);
        expected.dead_count = 1;

        let actual_id = apply(&mut actual, &Operator::XOR, -2, 3);
        // dbg!(&expected);
        // dbg!(&actual);
        assert_eq!(expected, actual);
        assert!(4 == actual_id);
    }

    #[test]
    fn apply_returns_complemented() {
        let mut actual = BDD::new();

        make(&mut actual, 1, -1, 1);
        make(&mut actual, 2, -1, 1);
        // dbg!(&actual);
        let mut expected = actual.clone();

        let expected_node = Rc::new(Vertex {var:1, lo: Some(-3), hi: Some(3)});
        expected.vertex_lookup.insert(expected_node.clone(), 4);
        expected.id_lookup.insert(4, expected_node);
        expected.computed_cache.insert(Expr {op: Operator::XOR, lhs: 2, rhs: 3}, -4);
        let rc = expected.ref_counts.get_mut(&2).unwrap();
        *rc = 0;
        expected.ref_counts.insert(4, 1);
        expected.dead_count = 1;

        let actual_id = apply(&mut actual, &Operator::XOR, 2, 3);
        // dbg!(&expected);
        // dbg!(&actual);
        assert_eq!(expected, actual);
        assert!(-4 == actual_id);
    }

    #[test]
    fn build_simple_test() {
        let eq = vec![Token::VAR('a'), Token::VAR('b'), Token::OP(Operator::AND), Token::OP(Operator::NOT)];
        let mut order: LinkedHashMap<char, usize> = LinkedHashMap::new();

        order.insert('a', 1);
        order.insert('b', 2);

        let mut actual_bdd = BDD::new();

        let mut expected_bdd = actual_bdd.clone();
        let lhs = make(&mut expected_bdd, 1, -1, 1);
        let rhs = make(&mut expected_bdd, 2, -1, 1);
        apply(&mut expected_bdd, &Operator::AND, lhs, rhs);

        actual_bdd = build_helper(actual_bdd, &eq, &order);
        assert_eq!(actual_bdd, expected_bdd);
    }

}