use crate::schema::FieldType;
use serde::{Deserialize, Serialize};

/// A definition of a field which a [`Document`] may have.
///
/// [`Document`]: crate::schema::Document
#[derive(Clone, Serialize, Deserialize)]
pub struct Field {
    /// The name of this field.
    pub name: String,
    /// The ID of this field.
    pub id: u16,
    /// The type of this field.
    pub field_type: FieldType,
}
