pub const MARKER_NULL: u8 = 0xC0;
pub const MARKER_FLOAT: u8 = 0xC1;
pub const MARKER_BOOLEAN_FALSE: u8 = 0xC2;
pub const MARKER_BOOLEAN_TRUE: u8 = 0xC3;
// tiny int is range of 0xF0~ 0x00 ~ 0x7F, value range [-16, 127]
pub const MARKER_INT8: u8 = 0xC8;
pub const MARKER_INT16: u8 = 0xC9;
pub const MARKER_INT32: u8 = 0xCA;
pub const MARKER_INT64: u8 = 0xCB;
// 255 bytes
pub const MARKER_SMALL_BYTES: u8 = 0xCC;
// 65,535 bytes
pub const MARKER_MEDIUM_BYTES: u8 = 0xCD;
// 2,147,483,647 bytes
pub const MARKER_LARGE_BYTES: u8 = 0xCE;

// shorter string marker is a range of 0x80~0x8F, 0x80 | string.len()
pub const MARKER_TINY_STRING_BASE: u8 = 0x80;
// 255 bytes
pub const MARKER_SMALL_STRING: u8 = 0xD0;
// 65,535 bytes
pub const MARKER_MEDIUM_STRING: u8 = 0xD1;
// 2,147,483,647 bytes
pub const MARKER_LARGE_STRING: u8 = 0xD2;

// list marker is range 0x90 ~ 0x9F,  MARKER_TINY_LIST_BASE | item.len
pub const MARKER_TINY_LIST_BASE: u8 = 0x90;
// 255 items
pub const MARKER_SMALL_LIST: u8 = 0xD4;
// 65,535 items
pub const MARKER_MEDIUM_LIST: u8 = 0xD5;
// 2,147,483,647 items
pub const MARKER_LARGE_LIST: u8 = 0xD6;

// map marker is range 0xA0 ~ 0xAF, MARKER_TINY_MAP_BASE | map.len
pub const MARKER_TINY_MAP_BASE: u8 = 0xA0;
// 255 entries
pub const MARKER_SMALL_MAP: u8 = 0xD8;
// 65535 entries
pub const MARKER_MEDIUM_MAP: u8 = 0xD9;
// 2 147 483 647 entries
pub const MARKER_LARGE_MAP: u8 = 0xDA;

// struct filed marker 0xB0 ~ 0xBF , MARKER_TINY_STRUCT_BASE | field.len
pub const MARKER_TINY_STRUCT_BASE: u8 = 0xB0;

pub const SIGNATURE_NODE: u8 = 0x4E;
pub const SIGNATURE_RELATIONSHIP: u8 = 0x52;
pub const SIGNATURE_UNBOUND_RELATIONSHIP: u8 = 0x72;
pub const SIGNATURE_PATH: u8 = 0x50;
pub const SIGNATURE_DATE: u8 = 0x44;
pub const SIGNATURE_TIME: u8 = 0x54;
pub const SIGNATURE_LOCAL_TIME: u8 = 0x74;
pub const SIGNATURE_DATE_TIME: u8 = 0x49;
pub const SIGNATURE_DATE_TIME_ZONE_ID: u8 = 0x69;
pub const SIGNATURE_LOCAL_DATE_TIME: u8 = 0x64;
pub const SIGNATURE_DURATION: u8 = 0x45;
pub const SIGNATURE_POINT_2D: u8 = 0x58;
pub const SIGNATURE_POINT_3D: u8 = 0x59;

pub const SIGNATURE_LEGACY_DATE_TIME: u8 = 0x46;
pub const SIGNATURE_LEGACY_DATE_TIME_ZONED_ID: u8 = 0x66;
