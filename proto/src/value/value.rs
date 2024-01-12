use std::collections::HashMap;
use std::hash::BuildHasher;
use std::path::Path;
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, Offset, TimeZone};
use chrono_tz::Tz;
use crate::serialization::BoltValue;
use crate::value::duration::Duration;
use crate::value::node::Node;
use crate::value::point_2d::Point2D;
use crate::value::point_3d::Point3D;
use crate::value::relationship::Relationship;
use crate::value::unbound_relationship::UnboundRelationship;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    Bytes(Vec<u8>),
    List(Vec<Value>),
    Map(HashMap<String, Value>),
    Null,
    String(String),
    Node(Node),
    Relationship(Relationship),
    Path(Path),
    UnboundRelationship(UnboundRelationship),

    // V2+-compatible value types
    Date(NaiveDate),              // A date without a time zone, i.e. LocalDate
    Time(NaiveTime, FixedOffset), // A time with UTC offset, i.e. OffsetTime
    DateTimeOffset(DateTime<FixedOffset>), // A date-time with UTC offset, i.e. OffsetDateTime
    DateTimeZoned(DateTime<Tz>),  // A date-time with time zone ID, i.e. ZonedDateTime
    LocalTime(NaiveTime),         // A time without time zone
    LocalDateTime(NaiveDateTime), // A date-time without time zone
    Duration(Duration),
    Point2D(Point2D),
    Point3D(Point3D),
}

impl Eq for Value {
    fn assert_receiver_is_total_eq(&self) {
        if let Value::Float(_) | Value::Point2D(_) | Value::Point3D(_) = self {
            panic!("{:?} does not impl Eq", self)
        }
    }
}

impl BoltValue for Value {

}

impl<T> From<Vec<T>> for Value
    where
        T: Into<Value>,
{
    fn from(value: Vec<T>) -> Self {
        Value::List(value.into_iter().map(T::into).collect())
    }
}

impl<K, V, S> From<HashMap<K, V, S>> for Value
    where
        K: Into<std::string::String>,
        V: Into<Value>,
        S: BuildHasher,
{
    fn from(value: HashMap<K, V, S>) -> Self {
        Value::Map(
            value
                .into_iter()
                .map(|(k, v)| (K::into(k), V::into(v)))
                .collect(),
        )
    }
}


// No timezone-aware time in chrono, so provide a separate conversion
impl<O: Offset> From<(NaiveTime, O)> for Value {
    fn from(pair: (NaiveTime, O)) -> Self {
        Value::Time(pair.0, pair.1.fix())
    }
}

impl<T: TimeZone> From<DateTime<T>> for Value {
    fn from(value: DateTime<T>) -> Self {
        Value::DateTimeOffset(DateTime::from_naive_utc_and_offset(value.naive_utc(), value.offset().fix()))
    }
}

// Can't decide between Offset or Zoned variant at runtime if using a T: TimeZone, so
// provide a separate conversion
impl From<(NaiveDateTime, chrono_tz::Tz)> for Value {
    fn from(pair: (NaiveDateTime, chrono_tz::Tz)) -> Self {
        Value::DateTimeZoned(pair.1.from_utc_datetime(&pair.0))
    }
}

impl<T: Into<Value>> From<Option<T>> for Value {
    fn from(option: Option<T>) -> Self {
        match option {
            Some(v) => v.into(),
            None => Value::Null,
        }
    }
}

impl From<std::time::Duration> for Value {
    fn from(value: std::time::Duration) -> Self {
        Value::Duration(Duration::from(value))
    }
}


#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use crate::value::value::Value;

    #[test]
    fn test_1() {
        let a = Value::Boolean(true);
    }

    #[test]
    fn test_date() {
        let a = Value::from(NaiveDate::MIN);
        println!("{:?}", a)
    }
}