use proto_macros::bolt_struct_derive;
use crate::value::node::Node;
use crate::value::unbound_relationship::UnboundRelationship;

#[bolt_struct_derive]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Path {
    pub(crate) nodes: Vec<Node>,
    pub(crate) relations: Vec<UnboundRelationship>,
    pub(crate) indices: Vec<i64>,
}

impl Path {
    pub fn new(
        nodes: Vec<Node>,
        relations: Vec<UnboundRelationship>,
        indices: Vec<i64>,
    ) -> Self {
        Self {
            nodes,
            relations,
            indices,
        }
    }

    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    pub fn relations(&self) -> &[UnboundRelationship] {
        &self.relations
    }

    pub fn indices(&self) -> &[i64] {
        &self.indices
    }
}