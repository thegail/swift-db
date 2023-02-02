use super::bare_document::{BareDocument, BareField, BareValue};
use crate::schema::{Document, EnumValue, FieldInstance, FieldType, FieldValue, Schema};
use crate::transfer::DeserializationError;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::de::Visitor;
use serde::Deserialize;
use std::io::Read;

impl Document {
    /// Reads a JSON serialized [`Document`] from
    /// a [`Read`].
    ///
    /// First deserializes a [`BareDocument`] with a
    /// [`serde_json::Deserializer`], then converts that
    /// into a [`Document`].
    pub fn from_reader(
        reader: impl Read,
        schema: &Schema,
    ) -> Result<Document, DeserializationError> {
        let mut deserializer = serde_json::Deserializer::from_reader(reader);
        let bare = BareDocument::deserialize(&mut deserializer)
            .map_err(DeserializationError::ParseError)?;
        Document::from_bare(bare, schema)
    }
}

struct DocumentVisitor;

impl<'de> Visitor<'de> for DocumentVisitor {
    type Value = BareDocument;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(formatter, "a map of key-value pairs")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut fields = Vec::with_capacity(map.size_hint().unwrap_or(0));
        while let Some((name, value)) = map.next_entry::<String, BareValue>()? {
            fields.push(BareField { name, value });
        }
        let document = BareDocument { fields };
        Ok(document)
    }
}

impl<'de> Deserialize<'de> for BareDocument {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(DocumentVisitor)
    }
}

struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor {
    type Value = BareValue;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "any value")
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BareValue::Integer(v as i64))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BareValue::Integer(v as i64))
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BareValue::Integer(v as i64))
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BareValue::Integer(v as i64))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BareValue::Integer(v as i64))
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BareValue::Integer(v as i64))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BareValue::Integer(v))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BareValue::Integer(i64::try_from(v).map_err(|_| {
            serde::de::Error::custom("u64 out of range")
        })?))
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BareValue::Float(v as f64))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BareValue::Float(v))
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BareValue::Bool(v))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BareValue::String(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BareValue::String(v.to_string()))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut values = Vec::with_capacity(seq.size_hint().unwrap_or(0));
        while let Some(value) = seq.next_element()? {
            values.push(value);
        }
        Ok(BareValue::Array(values))
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        Ok(BareValue::Object(Box::new(DocumentVisitor.visit_map(map)?)))
    }
}

impl<'de> Deserialize<'de> for BareValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor)
    }
}

impl Document {
    fn from_bare(bare: BareDocument, schema: &Schema) -> Result<Self, DeserializationError> {
        let fields: Result<Vec<FieldInstance>, DeserializationError> = bare
            .fields
            .into_iter()
            .map(|field| {
                let definition = schema
                    .fields
                    .iter()
                    .find(|f| f.name == field.name)
                    .ok_or_else(|| DeserializationError::FieldNotFound(field.name.clone()))?;
                let value = FieldValue::from_bare(field.value, &definition.field_type)?;
                let instance = FieldInstance {
                    id: definition.id,
                    value,
                };
                Ok(instance)
            })
            .collect();
        let document = Document {
            fields: fields?,
            schema: schema.clone(),
        };
        Ok(document)
    }
}

impl FieldValue {
    fn from_bare(bare: BareValue, definition: &FieldType) -> Result<Self, DeserializationError> {
        match definition {
            crate::schema::FieldType::Int => match bare {
                BareValue::Integer(i) => Ok(FieldValue::Int(
                    i32::try_from(i).map_err(|_| DeserializationError::Overflow(i))?,
                )),
                _ => Err(DeserializationError::FieldTypeMismatch),
            },
            crate::schema::FieldType::UInt => match bare {
                BareValue::Integer(i) => Ok(FieldValue::UInt(
                    u32::try_from(i).map_err(|_| DeserializationError::Overflow(i))?,
                )),
                _ => Err(DeserializationError::FieldTypeMismatch),
            },
            crate::schema::FieldType::Long => match bare {
                BareValue::Integer(i) => Ok(FieldValue::Long(i)),
                _ => Err(DeserializationError::FieldTypeMismatch),
            },
            crate::schema::FieldType::ULong => match bare {
                BareValue::Integer(i) => Ok(FieldValue::ULong(
                    u64::try_from(i).map_err(|_| DeserializationError::Overflow(i))?,
                )),
                _ => Err(DeserializationError::FieldTypeMismatch),
            },
            crate::schema::FieldType::Float => match bare {
                BareValue::Integer(i) => Ok(FieldValue::Float(i as f64)),
                BareValue::Float(f) => Ok(FieldValue::Float(f)),
                _ => Err(DeserializationError::FieldTypeMismatch),
            },
            crate::schema::FieldType::Bool => match bare {
                BareValue::Bool(b) => Ok(FieldValue::Bool(b)),
                _ => Err(DeserializationError::FieldTypeMismatch),
            },
            crate::schema::FieldType::DateTime => match bare {
                BareValue::Integer(i) => {
                    let naive_time = NaiveDateTime::from_timestamp(i, 0);
                    Ok(FieldValue::DateTime(DateTime::from_utc(naive_time, Utc)))
                }
                _ => Err(DeserializationError::FieldTypeMismatch),
            },
            crate::schema::FieldType::String => match bare {
                BareValue::String(s) => Ok(FieldValue::String(s)),
                _ => Err(DeserializationError::FieldTypeMismatch),
            },
            crate::schema::FieldType::ByteArray => match bare {
                BareValue::Array(a) => {
                    let values: Result<Vec<u8>, DeserializationError> =
                        a.into_iter()
                            .map(|value| match value {
                                BareValue::Integer(i) => Ok(u8::try_from(i)
                                    .map_err(|_| DeserializationError::Overflow(i))?),
                                _ => Err(DeserializationError::FieldTypeMismatch),
                            })
                            .collect();
                    Ok(FieldValue::ByteArray(values?))
                }
                _ => Err(DeserializationError::FieldTypeMismatch),
            },
            crate::schema::FieldType::Array(array_type) => match bare {
                BareValue::Array(a) => {
                    let values: Result<Vec<FieldValue>, DeserializationError> = a
                        .into_iter()
                        .map(|value| FieldValue::from_bare(value, &*array_type))
                        .collect();
                    Ok(FieldValue::Array(values?))
                }
                _ => Err(DeserializationError::FieldTypeMismatch),
            },
            crate::schema::FieldType::Object(sub_schema) => match bare {
                BareValue::Object(o) => Ok(FieldValue::Object(Box::new(Document::from_bare(
                    *o, sub_schema,
                )?))),
                _ => Err(DeserializationError::FieldTypeMismatch),
            },
            crate::schema::FieldType::Enum(cases) => match bare {
                BareValue::Object(mut o) => {
                    if o.fields.len() != 1 {
                        return Err(DeserializationError::FieldTypeMismatch);
                    }
                    let field = o.fields.pop().unwrap();
                    let case = cases
                        .iter()
                        .find(|case| case.name == field.name)
                        .ok_or(DeserializationError::CaseNotFound(field.name))?;
                    let mut associated_object = match field.value {
                        BareValue::Object(o) => o,
                        _ => return Err(DeserializationError::FieldTypeMismatch),
                    };
                    let associated_value = if associated_object.fields.is_empty() {
                        None
                    } else {
                        if associated_object.fields.len() != 1 {
                            return Err(DeserializationError::FieldTypeMismatch);
                        }
                        let field = associated_object.fields.pop().unwrap();
                        if field.name != "_0" {
                            return Err(DeserializationError::FieldTypeMismatch);
                        }
                        Some(FieldValue::from_bare(
                            field.value,
                            case.associated_value
                                .as_ref()
                                .ok_or(DeserializationError::FieldTypeMismatch)?,
                        )?)
                    };
                    Ok(FieldValue::Enum(Box::new(EnumValue {
                        case_id: case.id,
                        associated_value,
                    })))
                }
                _ => Err(DeserializationError::FieldTypeMismatch),
            },
        }
    }
}
