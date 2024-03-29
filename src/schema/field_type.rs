use crate::schema::Schema;
use crate::util::CaseID;
use serde::{Deserialize, Serialize};

/// The type of a [`Field`].
///
/// [`Field`]: crate::schema::Field
#[derive(Clone, Serialize, Deserialize)]
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
#[derive(Clone, Serialize, Deserialize)]
pub struct EnumCase {
    /// The name of this enum case.
    pub name: String,
    /// The ID of this enum case.
    pub id: CaseID,
    /// The associated type that is stored along
    /// with instances this caase.
    pub associated_value: Option<FieldType>,
}
