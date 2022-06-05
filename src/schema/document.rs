use super::{FieldInstance, Schema};

pub struct Document {
    pub schema: Schema,
    pub fields: Vec<FieldInstance>,
}
