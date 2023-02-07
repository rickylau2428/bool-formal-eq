use std::rc::Rc;

pub enum BDDNode {
    Sink(BDDSinkNode),
    Vertex(BDDVertex)
}

pub struct BDDSinkNode {
    val: usize
}

pub struct BDDVertexInner {
    var: usize,
    lo: BDDNode,
    hi: BDDNode
}

type BDDVertex = Rc<BDDVertexInner>;


