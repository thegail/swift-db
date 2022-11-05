use crate::backend::Query;

enum Statement {
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
