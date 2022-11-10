use super::bare_document::{BareDocument, BareField, BareValue};
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
