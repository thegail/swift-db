use crate::{schema::Field, util::SchemaID};
use serde::{Deserialize, Serialize};

/// A schema definition.
#[derive(Clone, Serialize, Deserialize)]
pub struct Schema {
    /// The name of the collection this schema defines.
    pub name: String,
    /// The ID of the collection this schema defines.
    pub id: SchemaID,
    /// The field definitions on this schema.
    pub fields: Vec<Field>,
}
