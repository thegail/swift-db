use super::bare_document::{BareDocument, BareField, BareValue};
use super::DeserializationError;
use crate::schema::{Document, FieldType, FieldValue};
use serde::ser::{SerializeMap, SerializeSeq, SerializeStruct};
use serde::{Serialize, Serializer};

impl Serialize for BareDocument {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut object = serializer.serialize_map(Some(self.fields.len()))?;
        for field in &self.fields {
            object.serialize_entry(&field.name, &field.value)?;
        }
        object.end()
    }
}

impl Serialize for BareValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            BareValue::Integer(i) => serializer.serialize_i64(*i),
            BareValue::Float(f) => serializer.serialize_f64(*f),
            BareValue::Bool(b) => serializer.serialize_bool(*b),
            BareValue::String(s) => serializer.serialize_str(s),
            BareValue::Array(array) => {
                let mut list = serializer.serialize_seq(Some(array.len()))?;
                for value in array {
                    list.serialize_element(&value)?;
                }
                list.end()
            }
            BareValue::Object(o) => o.serialize(serializer),
        }
    }
}

impl Document {
    fn to_bare(self) -> Result<BareDocument, DeserializationError> {
        let fields: Result<Vec<BareField>, DeserializationError> = self
            .fields
            .into_iter()
            .map(|field| {
                let definition = self
                    .schema
                    .fields
                    .iter()
                    .find(|f| f.id == field.id)
                    .ok_or_else(|| DeserializationError::FieldNotFound(field.id.to_string()))?;
                Ok(BareField {
                    name: definition.name,
                    value: field.value.to_bare()?,
                })
            })
            .collect();
        Ok(BareDocument { fields: fields? })
    }
}

impl FieldValue {
    fn to_bare(self) -> Result<BareValue, DeserializationError> {
        match self {
            FieldValue::Int(i) => Ok(BareValue::Integer(i as i64)),
            FieldValue::UInt(i) => Ok(BareValue::Integer(i as i64)),
            FieldValue::Long(i) => Ok(BareValue::Integer(i as i64)),
            FieldValue::ULong(i) => {
                if i > i64::MAX as u64 {
                    Err(DeserializationError::Overflow(0))
                } else {
                    Ok(BareValue::Integer(i as i64))
                }
            }
            FieldValue::Float(f) => Ok(BareValue::Float(f)),
            FieldValue::Bool(b) => Ok(BareValue::Bool(b)),
            FieldValue::DateTime(d) => Ok(BareValue::Integer(d.timestamp())),
            FieldValue::String(s) => Ok(BareValue::String(s)),
            FieldValue::ByteArray(b) => Ok(BareValue::Array(
                b.iter().map(|b| BareValue::Integer(*b as i64)).collect(),
            )),
            FieldValue::Array(a) => {
                let values: Result<Vec<BareValue>, DeserializationError> =
                    a.iter().map(|v| v.to_bare()).collect();
                Ok(BareValue::Array(values?))
            }
            FieldValue::Object(o) => Ok(BareValue::Object(Box::new(o.to_bare()?))),
            FieldValue::Enum(e) => todo!(),
        }
    }
}
