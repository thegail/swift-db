use super::parse_error::ParseError;
use crate::schema::{FieldType, FieldValue, Schema};
use crate::util::{FromByteSlice, PrimInt};
use chrono::{DateTime, NaiveDateTime, Utc};
use std::iter::Iterator;
use std::mem::size_of;

struct ArchiveParser {
    schema: Schema,
    data: Vec<u8>,
    ptr: usize,
}

impl ArchiveParser {
    fn read_field(&mut self) -> Result<FieldValue, ParseError> {
        let field_id = self.parse_int::<u16>();
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
            FieldType::Array(element) => {
                let element_copy = (*element).clone();
                Ok(FieldValue::Array(self.parse_array(element_copy)))
            }
            FieldType::Object(_schema) => Ok(FieldValue::Array(self.parse_object())),
        }
    }

    fn parse_int<T: PrimInt>(&mut self) -> T {
        let size = size_of::<T>();
        let bytes: T::Array = T::Array::from_slice(&self.data[self.ptr..(self.ptr + size)]);
        let value = T::from_be_bytes(bytes);
        self.ptr += size;
        value
    }

    fn parse_float(&mut self) -> f64 {
        let value = f64::from_be_bytes(self.data[self.ptr..(self.ptr + 8)].try_into().unwrap());
        self.ptr += 8;
        value
    }

    fn parse_bool(&mut self) -> bool {
        let value = self.data[self.ptr];
        self.ptr += 1;
        if value == 0 {
            false
        } else {
            true
        }
    }

    fn parse_datetime(&mut self) -> DateTime<Utc> {
        let timestamp = self.parse_int::<i64>();
        let naieve_time = NaiveDateTime::from_timestamp(timestamp, 0);
        DateTime::from_utc(naieve_time, Utc)
    }

    fn parse_byte_array(&mut self) -> Vec<u8> {
        let length = self.parse_int::<u32>() as usize;
        let value = (&self.data[self.ptr..(self.ptr + length)]).to_vec();
        self.ptr += length;
        value
    }

    fn parse_string(&mut self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.parse_byte_array())
    }

    fn parse_array(&mut self, element: Box<FieldType>) -> Vec<FieldValue> {
        vec![]
    }

    fn parse_object(&mut self) -> Vec<FieldValue> {
        vec![]
    }
}
