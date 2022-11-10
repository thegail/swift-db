use crate::schema::FieldType;

/// A definition of a field which a
/// [`Document`][crate::schema::Document] may have.
#[derive(Clone)]
pub struct Field {
    /// The name of this field.
    pub name: String,
    /// The ID of this field.
    pub id: u16,
    /// The type of this field.
    pub field_type: FieldType,
}
