use std::collections::HashMap;
use crate::value::value::Value;
use proto_macros::bolt_struct_derive;
use proto_common::marker::SIGNATURE_NODE;
use crate::serialization::{BoltStructure, BoltValue};

#[bolt_struct_derive]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Node {
    pub(crate) identity: i64,
    pub(crate) labels: Vec<String>,
    pub(crate) properties: HashMap<String, Value>,
    pub(crate) element_id: String,
}

impl Node {
    pub fn new(
        identity: i64,
        labels: Vec<String>,
        properties: HashMap<String, impl Into<Value>>,
        element_id: String,
    ) -> Self {
        Self {
            identity,
            labels,
            properties: properties.into_iter().map(|(k, v)| (k, v.into())).collect(),
            element_id,
        }
    }

    pub fn node_identity(&self) -> i64 {
        self.identity
    }

    pub fn labels(&self) -> &[String] {
        &self.labels
    }

    pub fn properties(&self) -> &HashMap<String, Value> {
        &self.properties
    }

    pub fn element_id(&self) -> &str {
        &self.element_id
    }
}

impl BoltStructure for Node {
    fn signature(&self) -> u8 {
        SIGNATURE_NODE
    }
}
