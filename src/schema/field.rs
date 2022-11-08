use crate::schema::FieldType;

#[derive(Clone)]
pub struct Field {
    pub name: String,
    pub id: u16,
    pub field_type: FieldType,
}
