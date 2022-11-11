use crate::backend::Query;
use crate::schema::Document;

/// An executable statement.
///
/// This is parsed from recieved bytes by [`parse`] and
/// [`build_statement`], then returned to the [`frontend`] to
/// be executed.
///
/// [`parse`]: crate::language::parse
/// [`build_statement`]: crate::language::build_statement
/// [`frontend`]: crate::frontend
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
