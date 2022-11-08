use crate::schema::Schema;

#[derive(Clone)]
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
    Enum(Vec<EnumCase>),
}

#[derive(Clone)]
pub struct EnumCase {
    pub name: String,
    pub id: u16,
    pub associated_value: Option<FieldType>,
}
