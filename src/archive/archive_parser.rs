use crate::archive::ParseError;
use crate::schema::{Document, EnumCase, EnumValue, FieldInstance, FieldType, FieldValue, Schema};
use crate::util::{CaseID, FieldID, FieldLength, FromByteSlice, PrimInt, SchemaID};
use chrono::{DateTime, NaiveDateTime, Utc};
use std::iter::Iterator;
use std::mem::size_of;

/// A parser for the archive binary serialization format.
///
/// When used by the [`Backend`],
/// an `ArchiveParser` takes a block of data produced by a
/// [`BlockFileIO`] manager and
/// deserializes a document from it.
///
/// [`Backend`]: crate::backend::Backend
/// [`BlockFileIO`]: crate::archive::BlockFileIO
pub struct ArchiveParser {
    schema: Schema,
    data: Vec<u8>,
    ptr: usize,
    fields_of_interest: Vec<FieldID>,
}

impl ArchiveParser {
    /// Creates a new [`ArchiveParser`] with a `Schema` and
    /// some bytes of data.
    pub fn new(schema: Schema, data: Vec<u8>, fields_of_interest: Vec<FieldID>) -> Self {
        ArchiveParser {
            schema,
            data,
            ptr: 0usize,
            fields_of_interest,
        }
    }

    /// Deserializes a [`Document`], checking the schema identifier
    /// against the [`ArchiveParser`]'s provided schema.
    pub fn read_document(&mut self) -> Result<Document, ParseError> {
        let schema_id = self.parse_int::<SchemaID>();
        if schema_id != self.schema.id {
            return Err(ParseError::SchemaMismatch);
        }
        self.read_subdocument()
    }

    /// Deserializes a [`Document`] without a schema identifier.
    pub fn read_subdocument(&mut self) -> Result<Document, ParseError> {
        let length = self.data.len();
        let mut fields: Vec<FieldInstance> = vec![];
        while self.ptr < length {
            match self.read_field()? {
                Some(field_instance) => fields.push(field_instance),
                None => {}
            }
        }
        Ok(Document {
            schema: self.schema.clone(),
            fields,
        })
    }

    fn read_field(&mut self) -> Result<Option<FieldInstance>, ParseError> {
        let field_id = self.parse_int::<FieldID>();
        let field = &self
            .schema
            .fields
            .iter()
            .find(|x| x.id == field_id)
            .ok_or(ParseError::UnknownFieldIdentifier)?
            .clone();
        if !self.fields_of_interest.contains(&field_id) {
            self.skip_field(&field.field_type)?;
            Ok(None)
        } else {
            Ok(Some(FieldInstance {
                id: field_id,
                value: self.parse_value(&field.field_type)?,
            }))
        }
    }

    fn parse_value(&mut self, field_type: &FieldType) -> Result<FieldValue, ParseError> {
        match field_type {
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
            FieldType::Array(element) => Ok(FieldValue::Array(self.parse_array(&*element)?)),
            FieldType::Object(schema) => {
                Ok(FieldValue::Object(Box::new(self.parse_object(&**schema)?)))
            }
            FieldType::Enum(cases) => Ok(FieldValue::Enum(Box::new(self.parse_enum(cases)?))),
        }
    }

    fn skip_field(&mut self, field_type: &FieldType) -> Result<(), ParseError> {
        match field_type {
            FieldType::Int => self.ptr += size_of::<i32>(),
            FieldType::UInt => self.ptr += size_of::<u32>(),
            FieldType::Long => self.ptr += size_of::<i64>(),
            FieldType::ULong => self.ptr += size_of::<u64>(),
            FieldType::Float => self.ptr += size_of::<f64>(),
            FieldType::Bool => self.ptr += size_of::<u8>(),
            FieldType::DateTime => self.ptr += size_of::<i64>(),
            FieldType::String => {
                let length = self.parse_int::<FieldLength>() as usize;
                self.ptr += length;
            }
            FieldType::ByteArray => {
                let length = self.parse_int::<FieldLength>() as usize;
                self.ptr += length;
            }
            FieldType::Array(_) => {
                let length = self.parse_int::<FieldLength>() as usize;
                self.ptr += length;
            }
            FieldType::Object(_) => {
                let length = self.parse_int::<FieldLength>() as usize;
                self.ptr += length;
            }
            FieldType::Enum(cases) => {
                let case_id = self.parse_int::<CaseID>();
                let enum_case = cases
                    .iter()
                    .find(|x| x.id == case_id)
                    .ok_or(ParseError::UnknownCaseIdentifier)?
                    .clone();
                match enum_case.associated_value {
                    Option::None => (),
                    Option::Some(value_type) => self.skip_field(&value_type)?,
                }
            }
        }
        Ok(())
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
        value != 0
    }

    fn parse_datetime(&mut self) -> DateTime<Utc> {
        let timestamp = self.parse_int::<i64>();
        let naive_time = NaiveDateTime::from_timestamp(timestamp, 0);
        DateTime::from_utc(naive_time, Utc)
    }

    fn parse_byte_array(&mut self) -> Vec<u8> {
        let length = self.parse_int::<FieldLength>() as usize;
        let value = (&self.data[self.ptr..(self.ptr + length)]).to_vec();
        self.ptr += length;
        value
    }

    fn parse_string(&mut self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.parse_byte_array())
    }

    fn parse_array(&mut self, element: &FieldType) -> Result<Vec<FieldValue>, ParseError> {
        let length = self.parse_int::<FieldLength>() as usize;
        let original_ptr = self.ptr;
        let mut values: Vec<FieldValue> = vec![];
        while self.ptr - original_ptr < length {
            values.push(self.parse_value(element)?);
        }
        Ok(values)
    }

    fn parse_object(&mut self, schema: &Schema) -> Result<Document, ParseError> {
        let bytes = self.parse_byte_array();
        let all_fields = schema.fields.iter().map(|f| f.id).collect();
        let mut parser = Self::new(schema.clone(), bytes, all_fields);
        parser.read_subdocument()
    }

    fn parse_enum(&mut self, cases: &[EnumCase]) -> Result<EnumValue, ParseError> {
        let case_id = self.parse_int::<CaseID>();
        let enum_case = cases
            .iter()
            .find(|x| x.id == case_id)
            .ok_or(ParseError::UnknownCaseIdentifier)?
            .clone();
        match enum_case.associated_value {
            Option::None => Ok(EnumValue {
                case_id,
                associated_value: None,
            }),
            Option::Some(value_type) => Ok(EnumValue {
                case_id,
                associated_value: Some(self.parse_value(&value_type)?),
            }),
        }
    }
}
