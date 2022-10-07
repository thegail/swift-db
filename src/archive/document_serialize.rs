use crate::schema::{Document, FieldValue};

impl Document {
    pub fn serialize(&self) -> Vec<u8> {
        let mut vector = Vec::<u8>::new();
        for field in self.fields.iter() {
            let bytes = field.id.to_be_bytes();
            vector.extend_from_slice(&bytes);
            vector.append(&mut field.value.serialize());
        }
        vec![]
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
            FieldValue::String(s) => s.as_bytes().to_vec(),
            FieldValue::ByteArray(b) => b.to_vec(),
            FieldValue::Array(_) => todo!(),
            FieldValue::Object(_) => todo!(),
            FieldValue::Enum(_) => todo!(),
        }
        .to_vec()
    }
}
