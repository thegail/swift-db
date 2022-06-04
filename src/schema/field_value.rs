use super::Schema;
use chrono::{DateTime, Utc};

pub enum FieldValue {
    Int(i32),
    UInt(u32),
    Long(i64),
    ULong(u64),
    Float(f64),
    Bool(bool),
    DateTime(DateTime<Utc>),
    String(String),
    ByteArray(Vec<u8>),
    Array(Vec<FieldValue>),
    // Object(Box<Schema>),
}
