use super::{FieldValue, Schema};

pub struct Document {
    schema: Schema,
    fields: Vec<FieldValue>,
}
