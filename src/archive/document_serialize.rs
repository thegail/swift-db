use crate::schema::{Document, FieldValue};

impl Document {
    pub fn serialize(&self) -> Vec<u8> {
        let mut vector = Vec::<u8>::new();
        let schema_bytes = self.schema.id.to_be_bytes();
        vector.extend_from_slice(&schema_bytes);
        for field in self.fields.iter() {
            let bytes = field.id.to_be_bytes();
            vector.extend_from_slice(&bytes);
            vector.append(&mut field.value.serialize());
        }
        vector
    }
}

impl FieldValue {
    fn serialize(&self) -> Vec<u8> {
        match self {
            FieldValue::Int(i) => i.to_be_bytes().to_vec(),
            FieldValue::UInt(i) => i.to_be_bytes().to_vec(),
            FieldValue::Long(i) => i.to_be_bytes().to_vec(),
            FieldValue::ULong(i) => i.to_be_bytes().to_vec(),
            FieldValue::Float(f) => f.to_be_bytes().to_vec(),
            FieldValue::Bool(b) => {
                if *b {
                    vec![1u8]
                } else {
                    vec![1u8]
                }
            }
            FieldValue::DateTime(d) => d.timestamp().to_be_bytes().to_vec(),
            FieldValue::String(s) => {
                let mut bytes = s.len().to_be_bytes().to_vec();
                bytes.extend_from_slice(s.as_bytes());
                bytes
            }
            FieldValue::ByteArray(b) => {
                let mut bytes = b.len().to_be_bytes().to_vec();
                bytes.extend_from_slice(b);
                bytes
            }
            FieldValue::Array(_) => todo!(),
            FieldValue::Object(_) => todo!(),
            FieldValue::Enum(_) => todo!(),
        }
        .to_vec()
    }
}
