use crate::schema::FieldValue;

/// A key/value pair for a field.
#[derive(Clone)]
pub struct FieldInstance {
    /// The ID of the field definition that this field
    /// conforms to.
    pub id: u16,
    /// The value stored in this field instance.
    pub value: FieldValue,
}
