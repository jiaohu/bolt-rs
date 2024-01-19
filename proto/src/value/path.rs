use proto_macros::bolt_struct_derive;
use crate::value::node::Node;
use crate::value::unbound_relationship::UnboundRelationship;
use proto_common::marker::SIGNATURE_PATH;
use crate::serialization::{BoltStructure, BoltValue};

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

impl BoltStructure for Path {
    fn signature(&self) -> u8 {
        SIGNATURE_PATH
    }
}