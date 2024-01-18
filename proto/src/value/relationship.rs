use std::collections::HashMap;
use proto_common::marker::SIGNATURE_RELATIONSHIP;
use proto_macros::bolt_struct_derive;
use crate::value::value::Value;
use crate::serialization::{BoltStructure, BoltValue};

#[bolt_struct_derive]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Relationship {
    pub(crate) identity: i64,
    pub(crate) start_identity: i64,
    pub(crate) end_identity: i64,
    pub(crate) rel_type: String,
    pub(crate) properties: HashMap<String, Value>,
    pub(crate) element_id: String,
    pub(crate) start_node_element_id: String,
    pub(crate) end_node_element_id: String,
}

impl Relationship {
    pub fn new(
        id: i64,
        start_id: i64,
        end_id: i64,
        rel_type: String,
        properties: HashMap<String, impl Into<Value>>,
        element_id: String,
        start_element_id: String,
        end_element_id: String,
    ) -> Self {
        Self {
            identity: id,
            start_identity: start_id,
            end_identity: end_id,
            rel_type,
            properties: properties.into_iter().map(|(k, v)| (k, v.into())).collect(),
            element_id,
            start_node_element_id: start_element_id,
            end_node_element_id: end_element_id,
        }
    }

    pub fn identity(&self) -> i64 {
        self.identity
    }

    pub fn start_identity(&self) -> i64 {
        self.start_identity
    }

    pub fn end_identity(&self) -> i64 {
        self.end_identity
    }

    pub fn rel_type(&self) -> &str {
        &self.rel_type
    }

    pub fn properties(&self) -> &HashMap<String, Value> {
        &self.properties
    }

    pub fn element_id(&self) -> &str {
        &self.element_id
    }

    pub fn start_node_element_id(&self) -> &str {
        &self.start_node_element_id
    }

    pub fn end_node_element_id(&self) -> &str {
        &self.end_node_element_id
    }
}

impl BoltStructure for Relationship {
    fn signature(&self) -> u8 {
        SIGNATURE_RELATIONSHIP
    }
}