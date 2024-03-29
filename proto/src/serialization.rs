use std::panic::UnwindSafe;
use bytes::{Buf, Bytes};
use proto_common::marker::MARKER_TINY_STRUCT_BASE;
use crate::result_type::{DeserializationError, DeserializeResult, SerializeResult};

pub(crate) trait BoltValue: Sized {
    fn marker(&self) -> SerializeResult<u8>;
    fn serialize(self) -> SerializeResult<Bytes>;
    fn deserialize<B: Buf + UnwindSafe>(bytes: B) -> DeserializeResult<(Self, B)>;
}


pub(crate) trait BoltStructure {
    fn signature(&self) -> u8;
}

/// Returns size and signature. Might panic - use this inside a catch_unwind block
pub(crate) fn get_structure_info(
    marker: u8,
    bytes: &mut impl Buf,
) -> DeserializeResult<(usize, u8)> {
    let size = match marker {
        marker if (MARKER_TINY_STRUCT_BASE..=(MARKER_TINY_STRUCT_BASE | 0x0F)).contains(&marker) => {
            0x0F & marker as usize
        }
        // MARKER_SMALL_STRUCT => bytes.get_u8() as usize,
        // MARKER_MEDIUM_STRUCT => bytes.get_u16() as usize,
        _ => return Err(DeserializationError::InvalidMarkerByte(marker)),
    };
    let signature = bytes.get_u8();
    Ok((size, signature))
}