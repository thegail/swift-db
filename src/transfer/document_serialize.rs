use crate::schema::{Document, Field, FieldType, FieldValue};
use std::io::Write;

impl Document {
    pub fn transfer_serialize(&self, out: &mut impl Write) -> Result<(), std::io::Error> {
        write!(out, "{{")?;
        serialize_list(out, self.fields.iter(), |field, out| {
            let definition = self
                .schema
                .fields
                .iter()
                .find(|f| f.id == field.id)
                .ok_or_else(|| todo!("field not found") as std::io::Error)?;
            write!(out, "\"{}\":", definition.name.escape_default())?;
            field.value.transfer_serialize(out, definition);
            Ok(())
        });
        write!(out, "}}")?;
        Ok(())
    }
}

impl FieldValue {
    fn transfer_serialize(
        &self,
        out: &mut impl Write,
        definition: &Field,
    ) -> Result<(), std::io::Error> {
        match self {
            FieldValue::Int(i) => write!(out, "{}", i),
            FieldValue::UInt(i) => write!(out, "{}", i),
            FieldValue::Long(i) => write!(out, "{}", i),
            FieldValue::ULong(i) => write!(out, "{}", i),
            FieldValue::Float(f) => write!(out, "{}", f),
            FieldValue::Bool(b) => write!(out, "{}", if *b { "true" } else { "false" }),
            FieldValue::DateTime(d) => write!(out, "{}", d.timestamp()),
            FieldValue::String(s) => write!(out, "\"{}\"", s.escape_default()),
            FieldValue::ByteArray(b) => {
                write!(out, "[")?;
                serialize_list(out, b.iter(), |byte, out| write!(out, "{}", byte))?;
                write!(out, "]")?;
                Ok(())
            }
            FieldValue::Array(a) => {
                write!(out, "[")?;
                serialize_list(out, a.iter(), |value, out| {
                    value.transfer_serialize(out, definition)
                })?;
                write!(out, "]")?;
                Ok(())
            }
            FieldValue::Object(o) => o.transfer_serialize(out),
            FieldValue::Enum(e) => {
                let cases = match &definition.field_type {
                    FieldType::Enum(cases) => cases,
                    _ => todo!("err handling"),
                };
                let case = cases
                    .iter()
                    .find(|c| c.id == e.case_id)
                    .ok_or_else(|| todo!("err handling") as std::io::Error)?;
                write!(out, "{{\"{}\":{{", case.name)?;
                if let Some(value) = &e.associated_value {
                    write!(out, "_0:")?;
                    value.transfer_serialize(out, definition)?;
                }
                write!(out, "}}")?;
                Ok(())
            }
        }
    }
}

fn serialize_list<I, F, W>(
    out: &mut W,
    values: impl Iterator<Item = I>,
    callback: F,
) -> Result<(), std::io::Error>
where
    W: Write,
    F: Fn(I, &mut W) -> Result<(), std::io::Error>,
{
    let mut is_first = true;
    for value in values {
        if is_first {
            is_first = false;
        } else {
            write!(out, ",")?;
        }
        callback(value, out)?;
    }
    Ok(())
}
