use crate::schema::Schema;

pub struct Selection {
    pub schema: Schema,
    pub position: usize,
}

pub struct ManySelection {
    pub schema: Schema,
    pub positions: Vec<usize>,
}
