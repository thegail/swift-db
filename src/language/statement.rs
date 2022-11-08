use crate::backend::Query;

pub enum Statement {
    Open {
        transaction: String,
    },
    Acquire {
        transaction: String,
    },
    Commit {
        transaction: String,
    },
    Close {
        transaction: String,
    },
    Select {
        identifier: String,
        transaction: String,
        // lock: LockType,
        query: Query,
    },
    // Read {
    //     selection: String,
    //     fields: Vec<u16>,
    // },
    ReadAll {
        selection: String,
    },
}
