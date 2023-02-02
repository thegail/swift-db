use crate::schema::{Document, FieldType, Schema};
use crate::util::CaseID;
use chrono::{DateTime, Utc};

/// An actual value stored in a
/// [`FieldInstance`].
///
/// [`FieldInstance`]: crate::schema::FieldInstance
#[derive(Clone)]
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
    Object(Box<Document>),
    Enum(Box<EnumValue>),
}

/// An instance of an enum case.
#[derive(Clone)]
pub struct EnumValue {
    /// The ID of the case definition to which this
    /// instance conforms.
    pub case_id: CaseID,
    /// The value assocaited with this case instance.
    pub associated_value: Option<FieldValue>,
}

impl FieldValue {
    /// Converts this field value to its corresponding
    /// [`FieldType`], stripping array/object/enum subtype
    /// information.
    pub fn simple_type(&self) -> FieldType {
        match self {
            Self::Int(_) => FieldType::Int,
            Self::UInt(_) => FieldType::UInt,
            Self::Long(_) => FieldType::Long,
            Self::ULong(_) => FieldType::ULong,
            Self::Float(_) => FieldType::Float,
            Self::Bool(_) => FieldType::Bool,
            Self::DateTime(_) => FieldType::DateTime,
            Self::String(_) => FieldType::String,
            Self::ByteArray(_) => FieldType::ByteArray,
            Self::Array(_) => FieldType::Array(Box::new(FieldType::Int)),
            Self::Object(_) => FieldType::Object(Box::new(Schema {
                name: String::new(),
                id: 0,
                fields: vec![],
            })),
            Self::Enum(_) => FieldType::Enum(vec![]),
        }
    }
}
