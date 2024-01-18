use std::collections::HashMap;
use proto_macros::bolt_struct_derive;
use crate::value::value::Value;
use proto_common::marker::SIGNATURE_UNBOUND_RELATIONSHIP;
use crate::serialization::{BoltStructure, BoltValue};

#[bolt_struct_derive]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnboundRelationship {
    pub(crate) identity: i64,
    pub(crate) rel_type: String,
    pub(crate) properties: HashMap<String, Value>,
    pub(crate) element_id: String,
}

impl UnboundRelationship {
    pub fn new(
        identity: i64,
        rel_type: String,
        properties: HashMap<String, impl Into<Value>>,
        element_id: String,
    ) -> Self {
        Self {
            identity,
            rel_type,
            properties: properties.into_iter().map(|(k, v)| (k, v.into())).collect(),
            element_id,
        }
    }

    pub fn identity(&self) -> i64 {
        self.identity
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
}

impl BoltStructure for UnboundRelationship {
    fn signature(&self) -> u8 {
        SIGNATURE_UNBOUND_RELATIONSHIP
    }
}