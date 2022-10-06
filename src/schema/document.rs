use super::{FieldInstance, Schema};

#[derive(Clone)]
pub struct Document {
    pub schema: Schema,
    pub fields: Vec<FieldInstance>,
}
