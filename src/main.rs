//! SwiftDB is a performant, ACID-compliant, stripped-down
//! document database built to work easily with Swift.
//!
//! See [`Database`] for a description of the architecture
//! of this program.
use database::{Configuration, LifecycleError};

// TODO remove all clones
// TODO pointer type aliases
mod archive;
mod backend;
mod database;
mod frontend;
mod language;
mod schema;
mod transfer;
mod util;

fn main() -> Result<(), LifecycleError> {
    let configuration = Configuration::from_environment()?;
    let database = configuration.make_database()?;
    database.start()?;
    Ok(())
}
