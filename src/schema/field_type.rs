use crate::schema::Schema;

/// The type of a [`Field`][crate::schema::Field].
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

/// A possible value for an enumerated field.
#[derive(Clone)]
pub struct EnumCase {
    /// The name of this enum case.
    pub name: String,
    /// The ID of this enum case.
    pub id: u16,
    /// The associated type that is stored along
    /// with instances this caase.
    pub associated_value: Option<FieldType>,
}
