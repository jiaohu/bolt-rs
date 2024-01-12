use proto_macros::bolt_struct_derive;

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