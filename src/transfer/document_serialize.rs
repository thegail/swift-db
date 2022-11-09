use crate::schema::{Document, FieldType, FieldValue};
use serde::ser::{SerializeMap, SerializeSeq, SerializeStruct};
use serde::{Serialize, Serializer};

// TODO: Ugly hack but works
struct ReferencedFieldValue<'a> {
    field: &'a FieldValue,
    definition: &'a FieldType,
}

impl Serialize for Document {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut object = serializer.serialize_map(Some(self.fields.len()))?;
        for field in &self.fields {
            let definition = self
                .schema
                .fields
                .iter()
                .find(|f| f.id == field.id)
                .ok_or(serde::ser::Error::custom("Field not found"))?;
            let referenced_value = ReferencedFieldValue {
                field: &field.value,
                definition: &definition.field_type,
            };
            object.serialize_entry(&definition.name, &referenced_value)?;
        }
        object.end()
    }
}

impl<'a> Serialize for ReferencedFieldValue<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.field {
            crate::schema::FieldValue::Int(i) => serializer.serialize_i32(*i),
            crate::schema::FieldValue::UInt(i) => serializer.serialize_u32(*i),
            crate::schema::FieldValue::Long(i) => serializer.serialize_i64(*i),
            crate::schema::FieldValue::ULong(i) => serializer.serialize_u64(*i),
            crate::schema::FieldValue::Float(f) => serializer.serialize_f64(*f),
            crate::schema::FieldValue::Bool(b) => serializer.serialize_bool(*b),
            crate::schema::FieldValue::DateTime(d) => serializer.serialize_i64(d.timestamp()),
            crate::schema::FieldValue::String(s) => serializer.serialize_str(&s),
            crate::schema::FieldValue::ByteArray(b) => serializer.serialize_bytes(&b),
            crate::schema::FieldValue::Array(array) => {
                let mut list = serializer.serialize_seq(Some(array.len()))?;
                let sub_definition = match self.definition {
                    FieldType::Array(sub) => sub,
                    _ => return Err(serde::ser::Error::custom("Field not found")),
                };
                for value in array {
                    let referenced_value = ReferencedFieldValue {
                        field: value,
                        definition: &*sub_definition,
                    };
                    list.serialize_element(&referenced_value)?;
                }
                list.end()
            }
            crate::schema::FieldValue::Object(o) => o.serialize(serializer),
            crate::schema::FieldValue::Enum(e) => {
                let mut object = serializer.serialize_map(Some(1))?;
                let cases = match self.definition {
                    FieldType::Enum(cases) => cases,
                    _ => return Err(serde::ser::Error::custom("Field not found")),
                };
                let case = cases
                    .iter()
                    .find(|c| c.id == e.case_id)
                    .ok_or_else(|| serde::ser::Error::custom("Enum case not found"))?;
                let associated_value = AssociatedValue {
                    field: &e.associated_value,
                    definition: &case.associated_value,
                };
                object.serialize_entry(&case.name, &associated_value)?;
                object.end()
            }
        }
    }
}

struct AssociatedValue<'a> {
    field: &'a Option<FieldValue>,
    definition: &'a Option<FieldType>,
}

impl<'a> Serialize for AssociatedValue<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut object = serializer.serialize_struct("Values", 1)?;
        if let Some(definition) = &self.definition {
            let field = self
                .field
                .as_ref()
                .ok_or(serde::ser::Error::custom("Field not found"))?;
            let referenced_value = ReferencedFieldValue {
                field: &field,
                definition: &definition,
            };
            object.serialize_field("_0", &referenced_value)?;
        }
        object.end()
    }
}
