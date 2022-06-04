use super::parse_error::ParseError;
use crate::schema::{FieldType, FieldValue, Schema};
use chrono::{DateTime, Utc};
use num_traits::int::PrimInt;
use std::iter::Iterator;
use std::mem::size_of;

struct ArchiveParser {
    schema: Schema,
    data: Vec<u8>,
    ptr: usize,
}

impl ArchiveParser {
    fn read_short(&mut self) -> u16 {
        let value = (self.data[self.ptr] << 8) as u16 + self.data[self.ptr + 1] as u16;
        self.ptr += 2;
        value
    }

    fn read_field(&mut self) -> Result<FieldValue, ParseError> {
        let field_id = self.read_short();
        let field = (&self.schema.fields)
            .into_iter()
            .find(|x| x.id == field_id)
            .ok_or(ParseError::UnknownFieldIdentifier)?;
        match &field.field_type {
            FieldType::Int => Ok(FieldValue::Int(self.parse_int::<i32>())),
            FieldType::UInt => Ok(FieldValue::UInt(self.parse_int::<u32>())),
            FieldType::Long => Ok(FieldValue::Long(self.parse_int::<i64>())),
            FieldType::ULong => Ok(FieldValue::ULong(self.parse_int::<u64>())),
            FieldType::Float => Ok(FieldValue::Float(self.parse_float())),
            FieldType::Bool => Ok(FieldValue::Bool(self.parse_bool())),
            FieldType::DateTime => Ok(FieldValue::DateTime(self.parse_datetime())),
            FieldType::String => Ok(FieldValue::String(
                self.parse_string().or(Err(ParseError::InvalidString))?,
            )),
            FieldType::ByteArray => Ok(FieldValue::ByteArray(self.parse_byte_array())),
            FieldType::Array(value) => Ok(FieldValue::Array(self.parse_array())),
            FieldType::Object(schema) => Ok(FieldValue::Array(self.parse_object())),
        }
    }

    fn parse_int<T: PrimInt>(&mut self) -> T {
        // let size = size_of::<T>();
        T::from(1).unwrap()
    }

    fn parse_float(&mut self) -> f64 {
        0.0
    }

    fn parse_bool(&mut self) -> bool {
        false
    }

    fn parse_datetime(&mut self) -> DateTime<Utc> {
        Utc::now()
    }

    fn parse_byte_array(&mut self) -> Vec<u8> {
        vec![]
    }

    fn parse_string(&mut self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.parse_byte_array())
    }

    fn parse_array(&mut self) -> Vec<FieldValue> {
        vec![]
    }

    fn parse_object(&mut self) -> Vec<FieldValue> {
        vec![]
    }
}
