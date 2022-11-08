use crate::schema::Schema;

#[derive(Clone)]
pub struct Selection {
    pub schema: Schema,
    pub position: usize,
}

#[derive(Clone)]
pub struct ManySelection {
    pub schema: Schema,
    pub positions: Vec<usize>,
}
