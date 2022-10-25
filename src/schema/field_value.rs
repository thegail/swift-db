use super::{Document, FieldType, Schema};
use chrono::{DateTime, Utc};

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

#[derive(Clone)]
pub struct EnumValue {
    pub case_id: u16,
    pub associated_value: Option<FieldValue>,
}

impl FieldValue {
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
