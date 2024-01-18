use proto_common::marker::SIGNATURE_TIME;
use proto_macros::bolt_struct_derive;
use crate::serialization::{BoltStructure, BoltValue};

#[bolt_struct_derive]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Time {
    pub(crate) nanoseconds: i64,
    pub(crate) tz_offset_seconds: i64
}

impl Time {
    pub fn new(nanoseconds: i64, tz_offset_seconds: i64) -> Self {
        Self {
            nanoseconds,
            tz_offset_seconds
        }
    }
}

impl BoltStructure for Time {
    fn signature(&self) -> u8 {
        SIGNATURE_TIME
    }
}