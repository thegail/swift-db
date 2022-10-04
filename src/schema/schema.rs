use super::Field;

#[derive(Clone)]
pub struct Schema {
    pub name: String,
    pub id: u64,
    pub fields: Vec<Field>,
}
