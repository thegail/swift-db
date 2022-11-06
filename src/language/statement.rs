use crate::backend::Query;

pub enum Statement {
    Select {
        identifier: String,
        transaction: String,
        // lock: LockType,
        query: Query,
    },
    Read {
        selection: String,
        fields: Vec<u16>,
    },
}
