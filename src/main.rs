//! SwiftDB is a performant, ACID-compliant, stripped-down
//! document database built to work easily with Swift.
//!
//! See [`Database`] for a description of the architecture
//! of this program.
use database::{Database, LifecycleError};
use test_stuff::test_schema;

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
    let database = Database::new("test.sdb".to_string(), vec![test_schema()])?;
    database.start()?;
    Ok(())
}

mod test_stuff {
    use crate::schema::{EnumCase, Field, FieldType, Schema};
    fn test_fields() -> Vec<Field> {
        vec![
            Field {
                name: "name".to_string(),
                id: 0x1,
                field_type: FieldType::String,
            },
            Field {
                name: "apple_count".to_string(),
                id: 0x2,
                field_type: FieldType::UInt,
            },
            Field {
                name: "money".to_string(),
                id: 0x3,
                field_type: FieldType::Float,
            },
            Field {
                name: "has_pet".to_string(),
                id: 0x4,
                field_type: FieldType::Bool,
            },
            Field {
                name: "birthday".to_string(),
                id: 0x5,
                field_type: FieldType::DateTime,
            },
            Field {
                name: "state".to_string(),
                id: 0x6,
                field_type: FieldType::Enum(vec![
                    EnumCase {
                        id: 0x1,
                        name: "happy".to_string(),
                        associated_value: None,
                    },
                    EnumCase {
                        id: 0x2,
                        name: "sad".to_string(),
                        associated_value: None,
                    },
                    EnumCase {
                        id: 0x3,
                        name: "angry".to_string(),
                        associated_value: None,
                    },
                ]),
            },
            Field {
                name: "coordinates".to_string(),
                id: 0x7,
                field_type: FieldType::Object(Box::new(coords_schema())),
            },
        ]
    }

    fn coords_schema() -> Schema {
        Schema {
            name: "coordinates".to_string(),
            id: 0x20,
            fields: vec![
                Field {
                    name: "x".to_string(),
                    id: 0x1,
                    field_type: FieldType::Int,
                },
                Field {
                    name: "y".to_string(),
                    id: 0x2,
                    field_type: FieldType::Int,
                },
            ],
        }
    }

    pub fn test_schema() -> Schema {
        Schema {
            name: "people".to_string(),
            id: 0x10,
            fields: test_fields(),
        }
    }
}
