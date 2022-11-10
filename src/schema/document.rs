use crate::schema::{FieldInstance, Schema};

/// A single record in the database.
#[derive(Clone)]
pub struct Document {
    /// The schema to which this `Document` conforms.
    pub schema: Schema,
    /// The fields/value pairs on this instance.
    pub fields: Vec<FieldInstance>,
}
