use crate::database::LifecycleError;
use crate::schema::Schema;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    schemas: Vec<Schema>,
    filename: String,
    port: u16,
}

impl Configuration {
    pub fn from_environment() -> Result<Self, LifecycleError> {
        let file = File::open("swift-db.json").map_err(LifecycleError::ConfigurationFileError)?;
        let object = serde_json::from_reader(file).map_err(LifecycleError::ConfigurationError)?;
        Ok(object)
    }
}
