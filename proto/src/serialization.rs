use std::panic::UnwindSafe;
use bytes::{Buf, Bytes};
use crate::result_type::{DeserializeResult, SerializeResult};

pub(crate) trait BoltValue : Sized {
    fn marker(&self) -> SerializeResult<u8>;
    fn serialize(self) -> SerializeResult<Bytes>;
    fn deserialize<B: Buf + UnwindSafe>(bytes: B) -> DeserializeResult<(Self, B)>;
}


pub(crate) trait BoltStructure {
    fn signature(&self) -> u8;
}