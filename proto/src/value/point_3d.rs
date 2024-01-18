use proto_macros::bolt_struct_derive;
use std::panic::UnwindSafe;
use bytes::{Buf, Bytes};
use proto_common::marker::SIGNATURE_POINT_3D;
use crate::result_type::{DeserializeResult, SerializeResult};
use crate::serialization::{BoltStructure, BoltValue};

#[bolt_struct_derive]
#[derive(Debug, Clone, PartialEq)]
pub struct Point3D {
    pub(crate) srid: i32,
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
}

impl Point3D {
    pub fn new(srid: i32, x: f64, y: f64, z: f64) -> Self {
        Self { srid, x, y, z }
    }

    pub fn srid(&self) -> i32 {
        self.srid
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }
}

impl BoltStructure for Point3D {
    fn signature(&self) -> u8 {
        SIGNATURE_POINT_3D
    }
}