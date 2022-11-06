use crate::backend::Query;

pub enum Statement {
    Select {
        identifier: String,
        transaction: String,
        // lock: LockType,
        condition: Query,
    },
    Read {
        selection: String,
        fields: Vec<u16>,
    },
}
