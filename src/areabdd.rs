// Region-based allocation for BDD
// Keep track of # of nodes in BDD; # of nodes with 0 references (dead)
// Come up with ratio (maybe half?) for when region should be re-allocated and things reassigned
// This may also be a good time to consider a BDD re-ordering? (If doing dynamic ordering)

use std::collections::HashMap;
use std::rc::Rc;
use crate::parser;

pub struct BDD {
    vertex_lookup: HashMap<Rc<Vertex>, usize>,
    id_lookup: HashMap<usize, Rc<Vertex>>,
    ref_counts: HashMap<usize, usize>,
    // num_dead: usize
    roots: Vec<Rc<Vertex>>,
    computed_cache: HashMap<Expr, usize>
}

#[derive(Eq, PartialEq, Hash)]
pub struct Vertex {
    var: VAL,
    lo: Option<usize>,
    hi: Option<usize>
}

#[derive(Eq, PartialEq, Hash)]
pub enum VAL {
    SINK(bool),
    VAR(usize)
}

#[derive(Eq, PartialEq, Hash)]
pub struct Expr {
    op: parser::Operator,
    lhs: usize,
    rhs: usize
}

impl BDD {
    pub fn new(input: &parser::SessionInput) -> Self {
        let mut vertex_lookup: HashMap<Rc<Vertex>, usize> = HashMap::with_capacity(50); // Arbitrary
        let mut id_lookup: HashMap<usize, Rc<Vertex>> = HashMap::with_capacity(50); // Arbitrary
        let ref_counts: HashMap<usize, usize> = HashMap::with_capacity(50); // Arbitrary
        let computed_cache: HashMap<Expr, usize> = HashMap::with_capacity(50); // Arbitrary

        // Initialize sinks
        let terminal_false = Rc::new(Vertex {var: VAL::SINK(false), lo: None, hi: None});
        let terminal_true = Rc::new(Vertex {var: VAL::SINK(true), lo: None, hi: None});

        vertex_lookup.insert(Rc::clone(&terminal_false), 0);
        vertex_lookup.insert(Rc::clone(&terminal_true), 1);

        id_lookup.insert(0, terminal_false);
        id_lookup.insert(1, terminal_true);

        let roots = Vec::new();

        Self {
            vertex_lookup,
            id_lookup,
            ref_counts,
            roots,
            computed_cache
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

    pub fn make(&mut self, var: usize, lo: usize, hi: usize) -> usize {
        if lo == hi {
            return lo;
        }

        let tmp = Rc::new(Vertex {var: VAL::VAR(var), lo: Some(lo), hi: Some(hi)});
        if let Some(ret) = self.vertex_lookup.get(&tmp) {
            return *ret;
        } else {
            let id = self.vertex_lookup.len();
            self.vertex_lookup.insert(Rc::clone(&tmp), id);
            self.id_lookup.insert(id, tmp);
            return id;
        }
    }

    pub fn apply(&mut self, op: parser::Operator, lhs: usize, rhs: usize) {
        let expr = Expr {op, lhs, rhs};
        if self.computed_cache.contains_key(&expr) {
            
            return 
        }

    }


    fn add_ref(&mut self, id: usize) {
        if let Some(count) = self.ref_counts.get_mut(&id) {
            *count += 1;
        } else {
            panic!("Attempted to increment ref_count of non-existent id");
        }
    }

    fn dec_ref(&mut self, id: usize) {
        if let Some(count) = self.ref_counts.get_mut(&id) {
            *count -= 1;
            if *count == 0 {
                let to_remove = self.id_lookup.get(count).unwrap();
                self.vertex_lookup.remove(to_remove);
                self.id_lookup.remove(&id);
                
            }
        } else {
            panic!("Attempted to increment ref_count of non-existent id");
        }
    }

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

