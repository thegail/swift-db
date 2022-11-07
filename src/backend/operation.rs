use super::{Query, Selection};

pub enum Operation {
    FindOne {
        query: Query,
    },
    Read {
        selection: Selection,
        fields: Vec<u16>,
    },
}
