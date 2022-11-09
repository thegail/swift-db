use crate::backend::Query;
use crate::schema::Document;

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
    Create {
        identifier: String,
        transaction: String,
        document: Document,
    },
    // Read {
    //     selection: String,
    //     fields: Vec<u16>,
    // },
    ReadAll {
        selection: String,
    },
}
