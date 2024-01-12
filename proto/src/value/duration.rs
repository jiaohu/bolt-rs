use proto_macros::bolt_struct_derive;

#[bolt_struct_derive]
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Duration {
    pub(crate) months: i64,
    pub(crate) days: i64,
    pub(crate) seconds: i64,
    pub(crate) nanoseconds: i32,
}

impl Duration {
    pub fn new(months: i64, days: i64, seconds: i64, nanoseconds: i32) -> Self {
        Self {
            months,
            days,
            seconds,
            nanoseconds,
        }
    }

    pub fn months(&self) -> i64 {
        self.months
    }

    pub fn days(&self) -> i64 {
        self.days
    }

    pub fn seconds(&self) -> i64 {
        self.seconds
    }

    pub fn nanoseconds(&self) -> i32 {
        self.nanoseconds
    }
}

impl From<std::time::Duration> for Duration {
    fn from(duration: std::time::Duration) -> Self {
        // This fits in an i64 because u64::MAX / (3600 * 24) < i64::MAX
        let days = (duration.as_secs() / (3600 * 24)) as i64;
        // This fits in an i64 since it will be less than 3600 * 24
        let seconds = (duration.as_secs() % (3600 * 24)) as i64;
        // This fits in an i32 because 0 <= nanos < 1e9 which is less than i32::MAX
        let nanos = duration.subsec_nanos() as i32;

        // Months are not well-defined in terms of seconds so let's not use them here
        Self {
            months: 0,
            days,
            seconds,
            nanoseconds: nanos,
        }
    }
}