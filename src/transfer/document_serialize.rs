use crate::schema::{Document, FieldValue, Schema};
use std::io::Write;

impl Document {
    fn serialize(&self, out: &mut impl Write, schema: &Schema) -> Result<(), std::io::Error> {
        write!(out, "{{")?;
        let is_first = true;
        for field in self.fields {
            if is_first {
                is_first = false;
            } else {
                write!(out, ",")?;
            }
            let definition = schema
                .fields
                .iter()
                .find(|f| f.id == field.id)
                .ok_or(todo!("field not found"))?;
            write!(out, "\"{}\":", definition.name.escape_default())?;
            field.value.serialize(out);
        }
        write!(out, "}}")?;
        Ok(())
    }
}

impl FieldValue {
    fn serialize(&self, out: &mut impl Write) -> Result<(), std::io::Error> {
        match self {
            FieldValue::Int(i) => write!(out, "{}", i),
            FieldValue::UInt(i) => write!(out, "{}", i),
            FieldValue::Long(i) => write!(out, "{}", i),
            FieldValue::ULong(i) => write!(out, "{}", i),
            FieldValue::Float(f) => write!(out, "{}", f),
            FieldValue::Bool(b) => write!(out, "{}", if b { "true" } else { "false" }),
            FieldValue::DateTime(d) => write!(out, "{}", d.timestamp()),
            FieldValue::String(s) => write!(out, "\"{}\"", s.escape_default()),
            FieldValue::ByteArray(b) => {
                write!(out, "[")?;
                let is_first = true;
                for byte in b {
                    if is_first {
                        is_first = false;
                    } else {
                        write!(out, ",")?;
                    }
                    write!(out, "{}", byte)?;
                }
                write!(out, "]")?;
                Ok(())
            }
            FieldValue::Array(_) => todo!(),
            FieldValue::Object(_) => todo!(),
            FieldValue::Enum(_) => todo!(),
        }
    }
}
