use proto_macros::bolt_struct_derive;
use proto_common::marker::SIGNATURE_POINT_2D;
use crate::serialization::{BoltStructure, BoltValue};

#[bolt_struct_derive]
#[derive(Debug, Clone, PartialEq)]
pub struct Point2D {
    pub(crate) srid: i32,
    pub(crate) x: f64,
    pub(crate) y: f64,
}

impl Point2D {
    pub fn new(srid: i32, x: f64, y: f64) -> Self {
        Self { srid, x, y }
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
}

impl BoltStructure for Point2D {
    fn signature(&self) -> u8 {
        SIGNATURE_POINT_2D
    }
}
