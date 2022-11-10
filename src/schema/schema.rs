use crate::schema::Field;

/// A schema definition.
#[derive(Clone)]
pub struct Schema {
    /// The name of the collection this schema defines.
    pub name: String,
    /// The ID of the collection this schema defines.
    pub id: u64,
    /// The field definitions on this schema.
    pub fields: Vec<Field>,
}
