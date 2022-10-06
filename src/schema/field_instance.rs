use super::FieldValue;

#[derive(Clone)]
pub struct FieldInstance {
    pub id: u16,
    pub value: FieldValue,
}
