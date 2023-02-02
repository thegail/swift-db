use super::bare_document::{BareDocument, BareField, BareValue};
use super::DeserializationError;
use crate::schema::{Document, FieldType, FieldValue};
use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Serialize, Serializer};
use serde_json::to_writer;
use std::io::Write;

impl Document {
    /// Writes a JSON serialization of a [`Document`]
    /// into a [`Write`].
    ///
    /// The [`Document`] is first converted into a [`BareDocument`],
    /// then serialized using [`serde_json::to_writer`].
    pub fn into_writer(self, mut writer: impl Write) -> Result<(), DeserializationError> {
        let bare = self.into_bare()?;
        writeln!(writer, "(ok document)").unwrap_or(());
        to_writer(writer, &bare).map_err(DeserializationError::ParseError)
    }
}

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
    fn into_bare(self) -> Result<BareDocument, DeserializationError> {
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
                    name: definition.name.clone(),
                    value: field.value.into_bare(&definition.field_type)?,
                })
            })
            .collect();
        Ok(BareDocument { fields: fields? })
    }
}

impl FieldValue {
    fn into_bare(self, definition: &FieldType) -> Result<BareValue, DeserializationError> {
        match self {
            FieldValue::Int(i) => Ok(BareValue::Integer(i as i64)),
            FieldValue::UInt(i) => Ok(BareValue::Integer(i as i64)),
            FieldValue::Long(i) => Ok(BareValue::Integer(i as i64)),
            FieldValue::ULong(i) => Ok(BareValue::Integer(
                i64::try_from(i).map_err(|_| DeserializationError::Overflow(0))?,
            )),
            FieldValue::Float(f) => Ok(BareValue::Float(f)),
            FieldValue::Bool(b) => Ok(BareValue::Bool(b)),
            FieldValue::DateTime(d) => Ok(BareValue::Integer(d.timestamp())),
            FieldValue::String(s) => Ok(BareValue::String(s)),
            FieldValue::ByteArray(b) => Ok(BareValue::Array(
                b.iter().map(|b| BareValue::Integer(*b as i64)).collect(),
            )),
            FieldValue::Array(a) => {
                let subtype = match definition {
                    FieldType::Array(s) => s,
                    _ => return Err(DeserializationError::FieldTypeMismatch),
                };
                let values: Result<Vec<BareValue>, DeserializationError> =
                    a.into_iter().map(|v| v.into_bare(&*subtype)).collect();
                Ok(BareValue::Array(values?))
            }
            FieldValue::Object(o) => Ok(BareValue::Object(Box::new(o.into_bare()?))),
            FieldValue::Enum(e) => {
                let cases = match definition {
                    FieldType::Enum(cases) => cases,
                    _ => return Err(DeserializationError::FieldTypeMismatch),
                };
                let case = cases
                    .iter()
                    .find(|c| c.id == e.case_id)
                    .ok_or_else(|| DeserializationError::CaseNotFound(e.case_id.to_string()))?;
                let associated_object = if let Some(associated_value) = e.associated_value {
                    if let Some(associated_definition) = &case.associated_value {
                        let bare_value = associated_value.into_bare(associated_definition)?;
                        BareDocument {
                            fields: vec![BareField {
                                name: "_0".to_string(),
                                value: bare_value,
                            }],
                        }
                    } else {
                        return Err(DeserializationError::FieldTypeMismatch);
                    }
                } else {
                    BareDocument { fields: Vec::new() }
                };
                Ok(BareValue::Object(Box::new(BareDocument {
                    fields: vec![BareField {
                        name: case.name.clone(),
                        value: BareValue::Object(Box::new(associated_object)),
                    }],
                })))
            }
        }
    }
}
