use super::Schema;

pub enum FieldType {
    Int,
    UInt,
    Long,
    ULong,
    Float,
    Bool,
    DateTime,
    String,
    ByteArray,
    Array(Box<FieldType>),
    Object(Box<Schema>),
}
