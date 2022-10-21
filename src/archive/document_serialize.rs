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
                    vec![0u8]
                }
            }
            FieldValue::DateTime(d) => d.timestamp().to_be_bytes().to_vec(),
            FieldValue::String(s) => {
                let s_bytes = s.as_bytes();
                let mut bytes = (s_bytes.len() as u32).to_be_bytes().to_vec();
                bytes.extend_from_slice(s_bytes);
                bytes
            }
            FieldValue::ByteArray(b) => {
                let mut bytes = (b.len() as u32).to_be_bytes().to_vec();
                bytes.extend_from_slice(b);
                bytes
            }
            FieldValue::Array(a) => {
                let mut bytes = [0u8; 4].to_vec();
                for val in a {
                    bytes.append(&mut val.serialize());
                }
                bytes.splice(0..3, (bytes.len() as u32 - 4).to_be_bytes());
                bytes
            }
            FieldValue::Object(o) => {
                let mut bytes = [0u8; 16].to_vec();
                bytes.append(&mut o.serialize());
                bytes.splice(0..7, (bytes.len() - 4).to_be_bytes());
                bytes.splice(8..15, o.schema.id.to_be_bytes());
                bytes
            }
            FieldValue::Enum(e) => {
                let mut bytes = e.case_id.to_be_bytes().to_vec();
                if let Some(associated_value) = &e.associated_value {
                    bytes.append(&mut associated_value.serialize());
                }
                bytes
            }
        }
        .to_vec()
    }
}
