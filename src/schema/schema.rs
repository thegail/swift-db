use super::Field;

#[derive(Clone)]
pub struct Schema {
    pub name: String,
    pub id: u128,
    pub fields: Vec<Field>,
}
