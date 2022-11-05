use crate::schema::Schema;

pub struct Selection {
    pub schema: Schema,
    pub position: usize,
}

pub struct MultipleSelection {
    pub schema: Schema,
    pub positions: Vec<usize>,
}
